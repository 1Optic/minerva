use chrono::DateTime;
use chrono::FixedOffset;

use clap::Parser;
use postgres_protocol::escape::escape_identifier;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStoreDeleteTimestamp {
    #[arg(
        help = "granularity for which to delete all data",
        long = "granularity"
    )]
    granularity: String,
    #[arg(
        help="timestamp for which to delete all data",
        value_parser=DateTime::parse_from_rfc3339
    )]
    timestamp: DateTime<FixedOffset>,
}

impl TrendStoreDeleteTimestamp {
    async fn delete_timestamp(&self) -> CmdResult {
        let client = connect_db().await?;

        for row in client.query("SELECT name FROM trend_directory.trend_store_part tsp JOIN trend_directory.trend_store ts ON ts.id = tsp.trend_store_id WHERE ts.granularity = $1::text::interval", &[&self.granularity]).await? {
            let table_name: &str = row.get(0);
            let query = format!("DELETE FROM trend.{} WHERE timestamp = $1", escape_identifier(table_name));
            client.query(&query, &[&self.timestamp]).await?;

            println!("Delete data in: '{table_name}'");
        }

        Ok(())
    }
}

impl Cmd for TrendStoreDeleteTimestamp {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.delete_timestamp())
    }
}
