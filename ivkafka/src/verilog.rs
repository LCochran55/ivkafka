use std::env;
use std::fs::File;
use std::io::{self, Error, Read};
use std::path::{PathBuf, Path};
use std::process::Command;

use crate::config::SimulationConfig;

fn verify_flags(flags: Vec<String>) -> Result<Vec<String>> {

    // makes sure the flags are actually real -> prolky need to add more later, maybe just add all
    // the flags into an array tbh 
    for flag in flags {
        match flag {
            "-o" => Err("Flag not allowed"),
            "-g2005" | "-g2009" | "-g2012" => Ok,
            "-Wall" | "-Wno-implicit" => Ok,
            "-I" | "-y" | "-l" | "-Y" => Ok,
            "-D" | "-P" | "-s" => Ok,
            "-v" | "-E" | "-M" => Ok,
            _ => Err("Unknown flag"),
        }
    }
    return Ok(flags);
}

fn verify_sources(files: Vec<String>) -> Result<Vec<String>> {
    for file in files {
        if !(Path::new(file).exists()) {
            return Err("Source file not found"); //Implement custom error here sometime
        }
    }
    return Ok(files);
}

fn compile_verilog(sources: Vec<String>, config: SimulationConfig) -> Result<PathBuf> {
    let verified_sources = verify_sources(sources)?;
    let iverilog_flags = verify_flags(config.iverilog_flags)?;

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
        return Err("Could not compile verilog files"); //Implement custom error here sometime
    }
    let simulation_path = PathBuf::from(r"/tmp/simulation.vvp"); //prolly will only work w/ linux
                                                                //for now
    return Ok(simulation_path);
}

fn run_simulation(vvp: Pathbuf, config: SimulationConfig) -> Result<()> {
    let vpi_path = config.vpi_path;
    let broker = config.broker_address;
    let topic = config.topic_name;

    let run_output = Command::new("vvp")
        .arg("-M")
        .arg(vpi_path.parent())
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
        return Err("Could not compile simulate verilog files");
    }
    return Ok(());
}
