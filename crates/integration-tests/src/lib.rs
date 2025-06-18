use std::sync::Once;

pub mod common;
pub mod compact_attribute;
pub mod create_kpi;
pub mod create_trigger;
pub mod default_alias;
pub mod entity_set;
pub mod get_entity_types;
pub mod initialize;
pub mod load_data;
pub mod trend_materialization;
pub mod trend_storage;
pub mod trend_value_information;
pub mod trigger_trigger;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        env_logger::init();
    });
}
