use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::borrow::Cow;

use log::info;

use async_trait::async_trait;
use clap::Parser;
use ratatui::widgets::{Block, Borders};
use serde::{Deserialize, Serialize};

use tokio::signal;
use ratatui::Frame;
use ratatui::prelude::*;
use futures::{future::FutureExt, StreamExt};
use tui_logger::*;
use unicode_segmentation::UnicodeSegmentation;

use minerva::cluster::{BuildImageProvider, MinervaCluster, MinervaClusterConfig};
use minerva::error::{Error, RuntimeError};
use minerva::instance::{load_instance_config, MinervaInstance};
use minerva::schema::migrate;
use minerva::trend_store::create_partitions;

use super::common::{Cmd, CmdResult, ENV_MINERVA_INSTANCE_ROOT};

#[derive(Debug, Parser, PartialEq)]
pub struct StartOpt {
    #[arg(long = "create-partitions", help = "create partitions")]
    create_partitions: bool,
    #[arg(long = "node-count", help = "number of worker nodes")]
    node_count: Option<u8>,
    #[arg(
        long = "with-definition",
        help = "Minerva instance definition root directory"
    )]
    instance_root: Option<PathBuf>,
    #[arg(long, help = "skip Minerva schema initialization", action)]
    no_schema_initialization: bool,
}

#[derive(Serialize, Deserialize)]
struct ClusterConfig {
    image_name: String,
    image_tag: String,
    path: String,
}

fn render(frame: &mut Frame) {
    let [output_area, log_area] = Layout::vertical([
        Constraint::Fill(50),
        Constraint::Fill(50),
    ])
    .areas(frame.area());

    frame.render_widget("Hello world", output_area);

    let formatter = Box::new(CustomFormatter::default());

    let logger_widget = tui_logger::TuiLoggerWidget::default()
        .block(
            Block::default()
                .title("logs")
                .borders(Borders::ALL)
        )
        .opt_formatter(Some(formatter));

    frame.render_widget(logger_widget, log_area);
}

async fn gui() {
    let mut reader = crossterm::event::EventStream::new();
    let mut terminal = ratatui::init();

    loop {
        let delay = tokio::time::sleep(tokio::time::Duration::from_millis(100));
        let draw_result = terminal.draw(render);

        match draw_result {
            Ok(_) => {},
            Err(e) => {
                println!("error in GUI: {e}");
            }
        }

        let event = reader.next().fuse();

        tokio::select! {
            _ = delay => { },
            maybe_event = event => {

                match maybe_event {
                    Some(Ok(crossterm::event::Event::Key(k))) => {
                        match k.code {
                            crossterm::event::KeyCode::Esc => {
                                break;
                            },
                            crossterm::event::KeyCode::Char('q') => {
                                break;
                            },
                            _ => {}
                        }
                    },
                    Some(Err(_)) => todo!(),
                    None => {
                        //break;
                    },
                    _ => todo!(),
                }
            }
        }
     }

     ratatui::restore();
}

#[async_trait]
impl Cmd for StartOpt {
    async fn run(&self) -> CmdResult {
        tui_logger::init_logger(tui_logger::LevelFilter::Info).unwrap();
        tui_logger::set_default_level(tui_logger::LevelFilter::Info);

        let gui_task_handle = tokio::spawn(gui());

        let minerva_instance_root_option: Option<PathBuf> = match &self.instance_root {
            Some(root) => Some(root.clone()),
            None => match env::var(ENV_MINERVA_INSTANCE_ROOT) {
                Ok(v) => Some(PathBuf::from(v)),
                Err(_) => None,
            },
        };

        info!("Starting containers");
        let node_count = self.node_count.unwrap_or(3);

        let config_file = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/postgresql.conf"));

        let cluster_config = if let Some(ref minerva_instance_root) = minerva_instance_root_option {
            let instance_config = load_instance_config(minerva_instance_root)
                .map_err(|e| format!("could not load instance config: {e}"))?;
            if let Some(docker_image_config) = instance_config.docker_image {
                let definition_file: PathBuf =
                    PathBuf::from_iter([minerva_instance_root, &docker_image_config.path]);

                MinervaClusterConfig {
                    image_provider: Box::new(BuildImageProvider {
                        image_name: docker_image_config.image_name.clone(),
                        image_tag: docker_image_config.image_tag.clone(),
                        definition_file,
                    }),
                    config_file,
                    worker_count: node_count,
                    ..Default::default()
                }
            } else {
                MinervaClusterConfig {
                    config_file,
                    worker_count: node_count,
                    ..Default::default()
                }
            }
        } else {
            MinervaClusterConfig {
                config_file,
                worker_count: node_count,
                ..Default::default()
            }
        };

        let cluster = MinervaCluster::start(&cluster_config).await?;

        info!("Started containers");

        let test_database = cluster.create_db().await.map_err(|e| {
            Error::Runtime(RuntimeError::from_msg(format!(
                "Could not create database: {e}"
            )))
        })?;

        info!("Connecting to controller");
        {
            info!("Creating Minerva schema");
            let mut client = test_database.connect().await?;

            let mut env = test_database.get_env();

            env.push(("PGSSLMODE".to_string(), "disable".to_string()));

            let query = format!("SET citus.shard_count = {};", cluster.size());

            client.execute(&query, &[]).await?;

            if !self.no_schema_initialization {
                let query = "SET citus.multi_shard_modify_mode TO 'sequential'";
                client.execute(query, &[]).await?;
                migrate(&mut client).await?;
                info!("Created Minerva schema");
            }

            if let Some(minerva_instance_root) = minerva_instance_root_option {
                info!(
                    "Initializing from '{}'",
                    minerva_instance_root.to_string_lossy()
                );

                let minerva_instance = MinervaInstance::load_from(&minerva_instance_root)?;

                env.push((
                    "MINERVA_INSTANCE_ROOT".to_string(),
                    minerva_instance_root.to_string_lossy().to_string(),
                ));

                minerva_instance.initialize(&mut client, &env).await?;

                if self.create_partitions {
                    create_partitions(&mut client, None).await?;
                }

                info!("Initialized");
            }

            let env_file_path = String::from("cluster.env");
            write_env_file(&env_file_path, &env);
        }

        info!("Minerva cluster is running (press CTRL-C to stop)");
        info!("Connect to the cluster on port {}", cluster.controller_port);
        info!("");
        info!(
            "  psql -h localhost -p {} -d {} -U postgres",
            cluster.controller_port, test_database.name
        );
        info!("");
        info!("or:");
        info!("");
        info!(
            "  PGHOST=localhost PGPORT={} PGDATABASE={} PGUSER=postgres PGSSLMODE=disable minerva",
            cluster.controller_port, test_database.name
        );

        tokio::select! {
            _ = gui_task_handle => {
            }
            _ = signal::ctrl_c() => {}
        }

        //signal::ctrl_c().await.map_err(|e| {
        //    Error::Runtime(format!("Could not start waiting for Ctrl-C: {e}").into())
        //})?;

        Ok(())
    }
}

fn write_env_file(file_path: &str, env: &[(String, String)]) {
    let env_file = File::create(file_path).expect("Could not create env file");

    let mut env_buf_writer = BufWriter::new(env_file);

    for (name, value) in env {
        env_buf_writer
            .write_fmt(format_args!("{name}={value}\n"))
            .unwrap();
    }
}

#[derive(Default)]
pub struct CustomFormatter {
    /// Base style of the widget
    pub style: Style,
    /// Level based style
    pub style_error: Option<Style>,
    pub style_warn: Option<Style>,
    pub style_debug: Option<Style>,
    pub style_trace: Option<Style>,
    pub style_info: Option<Style>,
    pub format_separator: char,
    pub format_timestamp: Option<String>,
    pub format_output_level: Option<TuiLoggerLevelOutput>,
    pub format_output_target: bool,
    pub format_output_file: bool,
    pub format_output_line: bool,
}

impl CustomFormatter {
    fn append_wrapped_line(
        &self,
        style: Style,
        indent: usize,
        lines: &mut Vec<Line>,
        line: &str,
        width: usize,
        with_indent: bool,
    ) {
        let mut p = 0;
        let mut wrap_len = width;
        if with_indent {
            wrap_len -= indent;
        }
        let space = " ".repeat(indent);
        let line_chars = line.graphemes(true).collect::<Vec<_>>();
        while p < line_chars.len() {
            let linelen = std::cmp::min(wrap_len, line_chars.len() - p);
            let subline = &line_chars[p..p + linelen];

            let mut spans: Vec<Span> = Vec::new();
            if wrap_len < width {
                // need indent
                spans.push(Span {
                    style,
                    content: Cow::Owned(space.to_string()),
                });
            }
            spans.push(Span {
                style,
                content: Cow::Owned(subline.iter().map(|x| x.to_string()).collect()),
            });
            let line = Line::from(spans);
            lines.push(line);

            p += linelen;
            // following lines need to be indented
            wrap_len = width - indent;
        }
    }
}

impl LogFormatter for CustomFormatter {
    fn min_width(&self) -> u16 {
        9 + 4
    }
    fn format(&self, width: usize, evt: &ExtLogRecord) -> Vec<Line> {
        let mut lines = Vec::new();
        let mut output = String::new();
        let (col_style, lev_long, lev_abbr, with_loc) = match evt.level {
            log::Level::Error => (self.style_error, "ERROR", "E", true),
            log::Level::Warn => (self.style_warn, "WARN ", "W", true),
            log::Level::Info => (self.style_info, "INFO ", "I", true),
            log::Level::Debug => (self.style_debug, "DEBUG", "D", true),
            log::Level::Trace => (self.style_trace, "TRACE", "T", true),
        };
        let col_style = col_style.unwrap_or(self.style);
        if let Some(fmt) = self.format_timestamp.as_ref() {
            output.push_str(&format!("{}", evt.timestamp.format(fmt)));
            output.push(self.format_separator);
        }
        match &self.format_output_level {
            None => {}
            Some(TuiLoggerLevelOutput::Abbreviated) => {
                output.push_str(lev_abbr);
                output.push(self.format_separator);
            }
            Some(TuiLoggerLevelOutput::Long) => {
                output.push_str(lev_long);
                output.push(self.format_separator);
            }
        }
        if self.format_output_target {
            output.push_str(&evt.target());
            output.push(self.format_separator);
        }
        if with_loc {
            if self.format_output_file {
                if let Some(file) = evt.file() {
                    output.push_str(file);
                    output.push(self.format_separator);
                }
            }
            if self.format_output_line {
                if let Some(line) = evt.line {
                    output.push_str(&format!("{}", line));
                    output.push(self.format_separator);
                }
            }
        }
        let mut sublines: Vec<&str> = evt.msg().lines().rev().collect();

        if !sublines.is_empty() {
            output.push_str(sublines.pop().unwrap());
            self.append_wrapped_line(col_style, 9, &mut lines, &output, width, false);

            for subline in sublines.iter().rev() {
                self.append_wrapped_line(col_style, 9, &mut lines, subline, width, true);
            }
        }

        lines
    }
}
