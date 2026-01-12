use std::time::Duration;

use chrono::DateTime;
use chrono::FixedOffset;

use chrono::Utc;
use clap::Parser;

use clap::Subcommand;

use minerva::trend_store::{
    columnarize_partitions, create_partitions, create_partitions_for_timestamp,
};
use postgres_protocol::escape::escape_identifier;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStorePartition {
    #[command(subcommand)]
    pub command: TrendStorePartitionCommands,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum TrendStorePartitionCommands {
    #[command(about = "create partitions")]
    Create(TrendStorePartitionCreate),
    #[command(about = "remove partitions")]
    Remove(TrendStorePartitionRemove),
    #[command(about = "change partitions to columnar storage")]
    Columnar(ColumnarizePartitions),
}

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStorePartitionRemove {
    #[arg(help = "do not really remove the partitions", short, long)]
    pretend: bool,
}

impl TrendStorePartitionRemove {
    async fn remove(&self) -> CmdResult {
        let client = connect_db().await?;

        let total_partition_count_query = "SELECT count(*) FROM trend_directory.partition";

        let row = client.query_one(total_partition_count_query, &[]).await?;

        let total_partition_count: i64 = row.try_get(0)?;

        let old_partitions_query = concat!(
            "SELECT p.id, p.name, p.from, p.to ",
            "FROM trend_directory.partition p ",
            "JOIN trend_directory.trend_store_part tsp ON tsp.id = p.trend_store_part_id ",
            "JOIN trend_directory.trend_store ts ON ts.id = tsp.trend_store_id ",
            "WHERE p.from < (now() - retention_period - partition_size - partition_size) ",
            "ORDER BY p.name"
        );

        let rows = client.query(old_partitions_query, &[]).await?;

        println!(
            "Found {} of {} partitions to be removed",
            rows.len(),
            total_partition_count
        );

        for row in rows {
            let partition_id: i32 = row.try_get(0)?;
            let partition_name: &str = row.try_get(1)?;
            let data_from: DateTime<Utc> = row.try_get(2)?;
            let data_to: DateTime<Utc> = row.try_get(3)?;

            if self.pretend {
                println!(
                    "Would have removed partition '{partition_name}' ({data_from} - {data_to})",
                );
            } else {
                let drop_query = format!(
                    "DROP TABLE trend_partition.{}",
                    escape_identifier(partition_name)
                );
                client.execute(&drop_query, &[]).await?;

                let remove_entry_query = "DELETE FROM trend_directory.partition WHERE id = $1";
                client.execute(remove_entry_query, &[&partition_id]).await?;

                println!("Removed partition '{partition_name}' ({data_from} - {data_to})",);
            }
        }

        Ok(())
    }
}

impl Cmd for TrendStorePartitionRemove {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.remove())
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct ColumnarizePartitions {}

impl ColumnarizePartitions {
    async fn columnarize(&self) -> CmdResult {
        let mut client = connect_db().await?;
        columnarize_partitions(&mut client).await?;
        Ok(())
    }
}

impl Cmd for ColumnarizePartitions {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.columnarize())
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStorePartitionCreate {
    #[arg(
        help="period for which to create partitions",
        long="ahead-interval",
        value_parser=humantime::parse_duration
    )]
    ahead_interval: Option<Duration>,
    #[arg(
        help="timestamp for which to create partitions",
        long="for-timestamp",
        value_parser=DateTime::parse_from_rfc3339
    )]
    for_timestamp: Option<DateTime<FixedOffset>>,
}

impl TrendStorePartitionCreate {
    async fn create(&self) -> CmdResult {
        let mut client = connect_db().await?;

        if let Some(for_timestamp) = self.for_timestamp {
            create_partitions_for_timestamp(&mut client, for_timestamp.into()).await?;
        } else {
            create_partitions(&mut client, self.ahead_interval).await?;
        }

        println!("Created partitions");
        Ok(())
    }
}

impl Cmd for TrendStorePartitionCreate {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.create())
    }
}
