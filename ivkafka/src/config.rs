//SimulationConfig

use std::path::PathBuf;

pub struct SimulationConfig {
    pub broker_address: String,
    pub topic_name: String,
    pub vpi_path: PathBuf,
    pub iverilog_flags: Vec<String>,
}
