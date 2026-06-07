use std::env;
use std::path::PathBuf;

use ivkafka::config::SimulationConfig;
use ivkafka::kafka_consumer;
use ivkafka::verilog;

use anyhow::Error;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    //IDEA:
    //CLI grab verilog files to be run by verilog
    //Initialize Kafka consumer
    //run and simulate the verilog files li-verilog style
    //Kafka consumer consumes the messages from li-verilog
    //
    //After that somehow do the feature stuff
    //
    //
    //read CLI args
    //main.rs builds a SimulationConfig from config.rs,
    //passes it to verilog.rs to compile and run,
    //and passes it to kafka_consumer.rs to set up the consumer

    let args: Vec<String> = env::args().collect();

    let mut file_sources = Vec::new();
    let mut flags = Vec::new();

    let mut broker = "localhost:9092".to_string();
    let mut topic = "vcd-topic".to_string();
    let mut vpi = "stream_producer.vpi".to_string();

    for arg in args[1..].iter() {
        match arg {
            arg if arg.starts_with("--broker=") => {
                broker = arg.strip_prefix("--broker=").unwrap().to_string()
            }
            arg if arg.starts_with("--topic=") => {
                topic = arg.strip_prefix("--topic=").unwrap().to_string()
            }
            arg if arg.starts_with("--vpi=") => {
                vpi = arg.strip_prefix("--vpi=").unwrap().to_string()
            }
            arg if arg.ends_with(".v") => file_sources.push(arg.to_string()),
            arg if arg.starts_with("-") => flags.push(arg.to_string()),
            _ => eprintln!("Unknown argument: {}", arg),
        }
    }

    let simulation = SimulationConfig {
        broker_address: broker,
        topic_name: topic,
        vpi_path: PathBuf::from(vpi),
        iverilog_flags: flags,
    };

    kafka_consumer::docker_startup().await?;
    let consumer = kafka_consumer::create_consumer(&simulation);

    let vvp = verilog::compile_verilog(file_sources, &simulation)
        .expect("Failed to compile verilog files");

    tokio::join!(kafka_consumer::poll_messages(consumer), async {
        verilog::run_simulation(vvp, &simulation);
    });

    Ok(())
}
