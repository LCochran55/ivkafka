use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use futures::StreamExt;

async fn run_consumer() {
    // Create a streaming consumer that yields messages as a stream
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", "iVerilog_stream")
        // Start from earliest message if no offset exists
        .set("auto.offset.reset", "earliest")
        // Enable automatic offset commits
        .set("enable.auto.commit", "true")
        .set("auto.commit.interval.ms", "5000")
        .create()
        .expect("Failed to create consumer");

    // Subscribe to one or more topics
    consumer
        .subscribe(&["vcd-topic"])
        .expect("Failed to subscribe");

    // Process messages as they arrive
    let mut stream = consumer.stream();
    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => {
                // Extract the payload as bytes and convert to string
                let payload = message.payload()
                    .map(|bytes| String::from_utf8_lossy(bytes).to_string())
                    .unwrap_or_default();
                
                let key = message.key()
                    .map(|bytes| String::from_utf8_lossy(bytes).to_string())
                    .unwrap_or_default();

                println!(
                    "Received message - Topic: {}, Partition: {}, Offset: {}, Key: {}, Payload: {}",
                    message.topic(),
                    message.partition(),
                    message.offset(),
                    key,
                    payload
                );
            }
            Err(err) => {
                eprintln!("Error receiving message: {}", err);
            }
        }
    }
}
