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

