use clap::Parser;

use minerva::instance::dump;

use super::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct DumpOpt {}

impl DumpOpt {
    async fn dump(&self) -> CmdResult {
        let mut client = connect_db().await?;

        dump(&mut client).await;

        Ok(())
    }
}

impl Cmd for DumpOpt {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.dump())
    }
}
