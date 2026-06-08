use crate::config::SimulationConfig;

use rdkafka::Message;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::CommitMode;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;

use bollard::Docker;
use bollard::container::LogOutput;
use bollard::models::*;

use tokio::sync::oneshot;
use tokio::time::{Duration, sleep, timeout};

use std::collections::HashMap;

use futures_util::stream::StreamExt;
use futures_util::stream::TryStreamExt;

use anyhow::Error;
use anyhow::Result;

pub fn create_consumer(config: &SimulationConfig) -> StreamConsumer {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &config.broker_address)
        .set("group.id", "iVerilog_stream")
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "true")
        .set("auto.commit.interval.ms", "5000")
        .create()
        .expect("Failed to create consumer");

    consumer
        .subscribe(&[&config.topic_name.as_str()])
        .expect("Failed to subscribe");

    return consumer;
}

pub async fn poll_messages(consumer: StreamConsumer) -> Result<()> {
    let mut stream = consumer.stream();
    loop {
        match timeout(Duration::from_secs(5), stream.next()).await {
            Ok(Some(result)) => match result {
                Ok(msg) => {
                    let payload = msg
                        .payload()
                        .map(|p| String::from_utf8_lossy(p).to_string())
                        .unwrap_or_default();
                    println!("{}", payload);
                }
                Err(rdkafka::error::KafkaError::MessageConsumption(
                    rdkafka::types::RDKafkaErrorCode::BrokerTransportFailure,
                )) => {
                    eprintln!("Retrying broker in 2s...");
                    sleep(Duration::from_secs(2)).await;
                    break;
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
            },
            Err(_) => {
                println!("No new messages, stream idle. Flushing and exiting.");
                consumer.commit_consumer_state(CommitMode::Sync);
                break;
            }
            Ok(None) => break,
        }
    }
    Ok(())
}

const KAFKA_IMAGE: &str = "confluentinc/cp-kafka:latest";
const CLUSTER_ID: &str = "NGU1ZGE5ZWYtMjg4My00NzFkLWFiOTYtMTFmMDY3YTAzNzVkCg==";

pub async fn docker_startup() -> Result<String> {
    let docker = Docker::connect_with_local_defaults().unwrap();

    match docker.inspect_container("ivkafka", None).await {
        Ok(info) => {
            let running = info.state.unwrap().running.unwrap();

            if running {
                return Ok(String::from("localhost:9092"));
            } else {
                docker.remove_container("ivkafka", None).await?;
                let sd1 = docker.clone();

                create_container(&sd1).await?;
                return Ok(String::from("localhost:9092"));
            }
        }
        Err(_) => {
            let sd1 = docker.clone();

            create_container(&sd1).await?;
            sleep(Duration::from_secs(10)).await;
            return Ok(String::from("localhost:9092"));
        }
    }
}

fn build_broker_config() -> ContainerCreateBody {
    let host_config = HostConfig {
        port_bindings: Some(HashMap::from([(
            "9092/tcp".to_string(),
            Some(vec![PortBinding {
                host_ip: Some("0.0.0.0".to_string()),
                host_port: Some("9092".to_string()),
            }]),
        )])),
        ..Default::default()
    };

    let broker1_config = ContainerCreateBody {
        image: Some(String::from(KAFKA_IMAGE)),
        host_config: Some(host_config),
        cmd: Some(vec![String::from("/etc/confluent/docker/run")]),
        env: Some(vec![
            String::from("KAFKA_ADVERTISED_LISTENERS=PLAINTEXT://localhost:9092"),
            String::from("KAFKA_LISTENERS=PLAINTEXT://0.0.0.0:9092,CONTROLLER://0.0.0.0:29093"),
            String::from("KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR=1"),
            String::from("KAFKA_NODE_ID=1"),
            String::from("KAFKA_PROCESS_ROLES=broker,controller"),
            String::from("KAFKA_CONTROLLER_QUORUM_VOTERS=1@localhost:29093"),
            String::from("KAFKA_CONTROLLER_LISTENER_NAMES=CONTROLLER"),
            String::from(
                "KAFKA_LISTENER_SECURITY_PROTOCOL_MAP=PLAINTEXT:PLAINTEXT,CONTROLLER:PLAINTEXT",
            ),
            format!("CLUSTER_ID={}", CLUSTER_ID),
        ]),
        ..Default::default()
    };

    broker1_config
}

async fn create_container(docker: &Docker) -> Result<()> {
    let broker1_config = build_broker_config();

    docker
        .create_image(
            Some(
                bollard::query_parameters::CreateImageOptionsBuilder::default()
                    .from_image(KAFKA_IMAGE)
                    .build(),
            ),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await?;

    docker
        .create_container(
            Some(
                bollard::query_parameters::CreateContainerOptionsBuilder::default()
                    .name("ivkafka")
                    .build(),
            ),
            broker1_config,
        )
        .await?;

    docker
        .start_container(
            "ivkafka",
            None::<bollard::query_parameters::StartContainerOptions>,
        )
        .await?;

    let mut stream1 = docker.logs(
        "ivkafka",
        Some(
            bollard::query_parameters::LogsOptionsBuilder::default()
                .follow(true)
                .stdout(true)
                .stderr(false)
                .build(),
        ),
    );

    while let Some(msg) = stream1.next().await {
        match msg {
            Ok(LogOutput::StdOut { message }) => {
                let log = String::from_utf8_lossy(&message);
                if log.contains("Kafka Server started") {
                    break;
                }
            }
            Err(e) => eprintln!("Log error: {}", e),
            _ => (),
        }
    }
    Ok(())
}
