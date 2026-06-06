//SimulationConfig
//so verilog.rs knows which broker to point the simulation at
//and kafka_consumer.rs knows the same one

use std::path::PathBuf;

pub struct SimulationConfig {
    broker_address: String,
    topic_name: String, 
    vpi_path: PathBuf, 
    iverilog_flags: Vec<String>
}



