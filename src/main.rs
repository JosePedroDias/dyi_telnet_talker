use std::{
    io::{BufRead, BufReader, Write}, net::TcpListener, thread
};

/*
brew install telnet
cargo run &
telnet 127.0.0.1 7878
*/

// multi user version
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        println!("Connection established!");

        /*let handle =*/ thread::spawn(move || {
            let mut stream = stream.unwrap();

            stream.write(b"Hello World\r\n").unwrap();

            let reader = BufReader::new(stream.try_clone().unwrap());
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        println!("Received: {}", line);

                        if line == "exit" {
                            break;
                        }

                        let reply = line.to_ascii_uppercase() + "\r\n";
                        stream.write(reply.as_bytes()).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error reading from stream: {}", e);
                        break;
                    }
                }
            }
            println!("finishing thread for this client");
        });

        //handle.join().unwrap(); // kills the outer thread as soon as the handled one ends
    }
    println!("all done");
}

// single consumer version
fn __main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Accept only one connection.
    if let Some(stream) = listener.incoming().next() {
        println!("Connection established!");

        let handle = thread::spawn(move || {
            let mut stream = stream.unwrap();

            stream.write(b"Hello World\r\n").unwrap();

            let reader = BufReader::new(stream.try_clone().unwrap());
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        println!("Received: {}", line);

                        if line == "exit" {
                            break;
                        }

                        let reply = line.to_ascii_uppercase() + "\r\n";
                        stream.write(reply.as_bytes()).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error reading from stream: {}", e);
                        break;
                    }
                }
            }
            println!("finishing");
        });

        // Wait for the spawned thread to finish.
        handle.join().unwrap();
    }
    println!("Server is shutting down.");
}
