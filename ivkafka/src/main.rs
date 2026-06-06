use std::env;
use std::path::PathBuf;

use crate::config::SimulationConfig;

pub fn main() {
    //IDEA:
    //CLI grab verilog files to be run by verilog
    //Initialize Kafka consumer
    //run and simulate the verilog files li-verilog style
    //Kafka consumer consumes the messages from li-verilog
    //
    //After that somehow do the feature stuff
    //
    //
    //read env vars and CLI args
    //main.rs builds a SimulationConfig from config.rs,
    //passes it to verilog.rs to compile and run,
    //and passes it to kafka_consumer.rs to set up the consumer

    let args: Vec<String> = env::args().collect();

    let mut file_sources = Vec::new();
    let mut flags = Vec::new();

    let mut broker = "localhost:9092";
    let mut topic = "vcd-topic";
    let mut vpi = "/path/to/stream_producer.vpi";

    for arg in arg[1..].iter() {
        match arg {
            arg if arg.starts_with("--broker=") => broker = arg.strip_prefix("--broker=").unwrap(),
            arg if arg.starts_with("--topic=") => topic = arg.strip_prefix("--topic=").unwrap(),
            arg if arg.starts_with("--vpi=") => vpi = arg.strip_prefix("--vpi=").unwrap(),
            arg if arg.ends_with(".v") => file_sources.push(arg),
            arg if arg.starts_with("-") => flags.push(arg),
            _ => eprintln!("Unknown argument: {}", arg),
        }
    }

    let simulation = SimulationConfig {
        broker_address: broker,
        topic_name: topic,
        vpi_path: PathBuf::from(vpi),
        iverilog_flags: flags,
    };
}
