use clap::{Parser, Subcommand};

use minerva::schema::{get_current_version, get_pending_migrations, migrate};

use super::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct SchemaMigrate {
    #[arg(long, help = "Only show pending migrations")]
    show_pending: bool,
}

impl SchemaMigrate {
    async fn migrate(&self) -> CmdResult {
        let mut client = connect_db().await?;

        if self.show_pending {
            let pending = get_pending_migrations(&mut client).await?;

            for (version, name) in pending {
                println!("{version} - {name}");
            }
        } else {
            let query = "SET citus.multi_shard_modify_mode TO 'sequential'";
            client.execute(query, &[]).await?;
            migrate(&mut client).await?;
        }

        Ok(())
    }
}

impl Cmd for SchemaMigrate {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.migrate())
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct SchemaCurrentVersion {}

impl SchemaCurrentVersion {
    async fn show_current_version(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let version = get_current_version(&mut client).await?;

        if let Some(version_num) = version {
            println!("Current Minerva schema version: {version_num}");
        }

        Ok(())
    }
}

impl Cmd for SchemaCurrentVersion {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.show_current_version())
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct SchemaOpt {
    #[command(subcommand)]
    command: SchemaOptCommands,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum SchemaOptCommands {
    #[command(about = "migrate schema to latest version")]
    Migrate(SchemaMigrate),
    #[command(about = "show current version")]
    CurrentVersion(SchemaCurrentVersion),
}

impl SchemaOpt {
    /// # Errors
    ///
    /// Will return `Err` if a subcommand returns an error.
    pub fn run(&self) -> CmdResult {
        match &self.command {
            SchemaOptCommands::Migrate(migrate) => migrate.run(),
            SchemaOptCommands::CurrentVersion(current_version) => current_version.run(),
        }
    }
}
