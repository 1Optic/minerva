use clap::Parser;

use clap::Subcommand;

use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};

use minerva::trend_store::analyze_trend_store_part;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStorePartAnalyze {
    #[arg(help = "name of trend store part")]
    name: String,
}

impl TrendStorePartAnalyze {
    async fn analyze(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let result = analyze_trend_store_part(&mut client, &self.name).await?;

        println!("Analyzed '{}'", self.name);

        let mut table = Table::new();
        table.style = TableStyle::thin();
        table.separate_rows = false;

        table.add_row(Row::new(vec![
            TableCell::new("Name"),
            TableCell::new("Min"),
            TableCell::new("Max"),
        ]));

        for stat in result.trend_stats {
            table.add_row(Row::new(vec![
                TableCell::new(&stat.name),
                TableCell::builder(stat.min_value.unwrap_or("N/A".into()))
                    .col_span(1)
                    .alignment(Alignment::Right)
                    .build(),
                TableCell::builder(stat.max_value.unwrap_or("N/A".into()))
                    .col_span(1)
                    .alignment(Alignment::Right)
                    .build(),
            ]));
        }

        println!("{}", table.render());

        Ok(())
    }
}

impl Cmd for TrendStorePartAnalyze {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.analyze())
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStorePartOpt {
    #[command(subcommand)]
    pub command: TrendStorePartOptCommands,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum TrendStorePartOptCommands {
    #[command(about = "analyze range of values for trends in a trend store part")]
    Analyze(TrendStorePartAnalyze),
}
