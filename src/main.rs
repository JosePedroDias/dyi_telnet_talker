use std::{
    io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, thread
};

/*
brew install telnet
cargo run &
telnet 127.0.0.1 7878
*/

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        //let stream = stream.unwrap();

        println!("Connection established!");
        //handle_connection(stream);

        thread::spawn(|| {
            let mut stream = stream.unwrap();

            stream.write(b"Hello World\r\n").unwrap();

            //let reader = BufReader::new(stream);
            let reader = BufReader::new(stream.try_clone().unwrap());
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        println!("Received: {}", line);
                        let reply = line.to_ascii_uppercase() + "\r\n";
                        stream.write(reply.as_bytes()).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error reading from stream: {}", e);
                        break;
                    }
                }
            }
        });
    }
}

fn _handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}
