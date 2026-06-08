//Feature selection Implementation
//
//Python has better feasture selection things I think for later feature selection implementation
//So implementation plan:
//
//RUST:
//Rust has an mpsc channel which will take the streamed messages from poll_messages as theyre
//streaming
//read from this channel and write the lines to python stdin
//when kafka is over, mspc channel will close, and will signal to python it got to the end of the simulation
//then rust will wait for the returned things from python
//
//PYTHON:
//Python reads what Rust is giving it thru stdin 
//then python starts the clustering:
//  Reads the vcd header to understand what the symbols is what register -> map symbols to their registers
//  computes hamming data as VCD lines arrive
//when python recieves it got to the end of the file then:
//  clusters registers based off of hamming distance (?) idk
//  then when python is done I think it returns to rust thru stdout

use tokio::sync::mpsc;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

use std::process::Stdio;

use anyhow::Result;


pub async fn mpsc_writer(mut rx: mpsc::Receiver<String>) -> Result<() >{
     let mut child = Command::new("python3")
        .arg("process_stdin.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped()) // capture python's output for later
        .spawn()?;

    let mut child_stdin = child
        .stdin
        .take()
        .expect("Failed to open python stdin");

    while let Some(msg) = rx.recv().await {
        child_stdin
            .write(format!("{}\n", msg).as_bytes())
            .await?;

    }

    drop(child_stdin);
    child.wait().await?;

    Ok(())
}
