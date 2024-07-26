use std::{
    io::{BufRead, BufReader, Write}, net::TcpListener, sync::{Arc, Mutex}, thread
};

/*
brew install telnet
cargo run &
telnet 127.0.0.1 7878
*/

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        println!("Connection established!");

        let names = vec![];
        let names = Mutex::new(names);
        let names = Arc::new(names);

        thread::spawn(move || {
            let names = Arc::new(names);

            let mut stream = stream.unwrap();

            stream.write(b"What is your name?\r\n").unwrap();

            let mut line_no = 0;
            let reader = BufReader::new(stream.try_clone().unwrap());
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        line_no += 1;

                        println!("Received: {}", line);

                        if line == "exit" {
                            stream.write(b"Bye!\r\n").unwrap();
                            break;
                        }

                        let mut reply = line.to_ascii_uppercase() + "\r\n";
                        if line_no == 1 {
                            let my_name = line.clone();
                            reply = format!("Hello, {}!\r\n", my_name);

                            let mut names = names.lock().unwrap();
                            names.push(my_name.clone());

                            let joined_names = names.join(", ");
                            reply = format!("{}Users here: {}\r\n", reply, joined_names);
                        }

                        stream.write(reply.as_bytes()).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error reading from stream: {}", e);
                        break;
                    }
                }
            }
            println!("finishing a thread");
        });
    }
}
