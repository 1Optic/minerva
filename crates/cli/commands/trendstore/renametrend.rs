use async_trait::async_trait;
use clap::Parser;

use minerva::error::{Error, RuntimeError};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStoreRenameTrend {
    #[arg(help = "name of trend store part")]
    trend_store_part: String,
    #[arg(help = "current name")]
    from: String,
    #[arg(help = "new name")]
    to: String,
}

#[async_trait]
impl Cmd for TrendStoreRenameTrend {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let transaction = client.transaction().await?;

        let query = concat!(
            "UPDATE trend_directory.table_trend ",
            "SET name = $3 ",
            "FROM trend_directory.trend_store_part tsp ",
            "WHERE tsp.id = trend_store_part_id AND tsp.name = $1 AND table_trend.name = $2"
        );

        let update_count = transaction
            .execute(query, &[&self.trend_store_part, &self.from, &self.to])
            .await
            .map_err(|e| {
                Error::Runtime(RuntimeError {
                    msg: format!(
                        "Error renaming trend '{}' of trend store part '{}': {e}",
                        &self.from, &self.trend_store_part
                    ),
                })
            })?;

        if update_count == 0 {
            return Err(Error::Runtime(RuntimeError {
                msg: format!(
                    "No trend found matching trend store part name '{}' and name '{}'",
                    &self.trend_store_part, &self.from
                ),
            }));
        }

        transaction.commit().await?;

        println!(
            "Renamed {}.{} -> {}.{}",
            self.trend_store_part, self.from, self.trend_store_part, self.to
        );

        Ok(())
    }
}
