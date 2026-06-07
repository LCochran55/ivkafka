use std::env;
use std::fs::File;
use std::io::{self, Error, Read};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::SimulationConfig;

pub fn verify_flags(flags: Vec<String>) -> Result<Vec<String>, String> {
    // makes sure the flags are actually real ->  need to add more flags later, maybe just add all
    // the flags into an array tbh

    let flag_array = vec![
        "-g2005",
        "-g2009",
        "-g2012",
        "-Wall",
        "-Wno-implicit",
        "-I",
        "-y",
        "-l",
        "-Y",
        "-D",
        "-P",
        "-s",
        "-v",
        "-E",
        "-M",
    ];
    for flag in &flags {
        match flag.as_str() {
            "-o" => return Err("Flag not allowed".to_string()),
            f if flags.contains(&f.to_string()) => {}
            _ => return Err("Unknown flag".to_string()),
        };
    }
    return Ok(flags);
}

pub fn verify_sources(files: Vec<String>) -> Result<Vec<String>, String> {
    for file in &files {
        if !(Path::new(&file).exists()) {
            return Err("Source file not found".to_string()); //Implement custom error here sometime
        }
    }
    return Ok(files);
}

pub fn compile_verilog(sources: Vec<String>, config: &SimulationConfig) -> Result<PathBuf, String> {
    let verified_sources = verify_sources(sources)?;
    let iverilog_flags = verify_flags(config.iverilog_flags.clone())?;

    let compile_output = Command::new("iverilog")
        .args(iverilog_flags)
        .arg("-o")
        .arg("/tmp/simulation.vvp")
        .args(verified_sources)
        .output()
        .expect("Failed to compile Verilog file");

    if !compile_output.status.success() {
        eprintln!(
            "Compilation failed: {}",
            String::from_utf8_lossy(&compile_output.stderr)
        );
        return Err("Could not compile verilog files".to_string()); //Implement custom error here sometime
    }
    let simulation_path = PathBuf::from(r"/tmp/simulation.vvp"); //prolly will only work w/ linux
    //for now
    return Ok(simulation_path);
}

pub fn run_simulation(vvp: PathBuf, config: &SimulationConfig) -> Result<(), String> {
    let vpi_path = &config.vpi_path;
    let broker = &config.broker_address;
    let topic = &config.topic_name;

    let run_output = Command::new("vvp")
        .arg("-M")
        .arg(vpi_path)
        .arg("-m")
        .arg("stream_producer")
        .arg(vvp)
        .env("KAFKA_BROKER", broker)
        .env("KAFKA_TOPIC", topic)
        .output()
        .expect("Failed to run the compiled file");

    if !run_output.status.success() {
        eprintln!(
            "Execution failed: {}",
            String::from_utf8_lossy(&run_output.stderr)
        );
        return Err("Could not compile simulate verilog files".to_string());
    }
    return Ok(());
}
