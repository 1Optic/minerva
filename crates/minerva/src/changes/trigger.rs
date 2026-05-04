use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, GenericClient};

use crate::change::{Change, ChangeResult, Changed, InformationOption, MinervaObjectRef};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddKpiData {
    pub trigger_name: String,
    pub kpi_column: String,
    pub data_type: String,
}

impl Display for AddKpiData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AddKpiData {{ trigger_name: {}, kpi_column: {}}}",
            self.trigger_name, self.kpi_column
        )
    }
}

#[async_trait]
#[typetag::serde]
impl Change for AddKpiData {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let query = ""
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ChangeKpiData {
    pub trigger_name: String,
    pub kpi_column: String,
    pub data_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RemoveKpiData {
    pub trigger_name: String,
    pub kpi_column: String,
}
