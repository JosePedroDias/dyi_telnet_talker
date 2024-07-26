use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    sync::{Arc, RwLock},
    thread,
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

        // shared info
        let names = Arc::new(RwLock::new(vec![]));
        let topic = Arc::new(RwLock::new(String::from("unset")));

        thread::spawn(move || {
            let names = Arc::new(names);

            let mut stream = stream.unwrap();

            stream.write(b"What is your name?\r\n").unwrap();

            let mut line_no = 0;
            let reader = BufReader::new(stream.try_clone().unwrap());
            for input in reader.lines() {
                match input {
                    Ok(input) => {
                        line_no += 1;

                        println!("Received: {}", input);

                        let mut reply;
                        match input.as_str() {
                            "exit" => {
                                stream.write(b"Bye!\r\n").unwrap();
                                break;
                            }
                            "list users" => {
                                let names = names.read().unwrap();
                                let joined_names = names.join(", ");
                                reply = format!("Users online: {}", joined_names);
                            }
                            "get topic" => {
                                let topic = topic.read().unwrap().clone();
                                reply = format!("Current topic: {}", topic);
                            }
                            _ if line_no == 1 => {
                                let my_name = input.clone();
                                reply = format!("Hello, {}!", my_name);
                                let mut names = names.write().unwrap();
                                names.push(my_name.clone());
                            }
                            _ => {
                                let tokens = input.split_whitespace().collect::<Vec<_>>();
                                let first = tokens.get(0);
                                let second = tokens.get(1);

                                println!("tokens: {:?} {:?}", first, second);

                                if first == Some(&"set") && second == Some(&"topic") {
                                    let new_topic = tokens[2].to_string();
                                    let mut topic = topic.write().unwrap();
                                    *topic = new_topic.clone();
                                    reply = format!("Topic was set to to {}", new_topic);
                                } else {
                                    reply = input.to_ascii_uppercase();
                                }
                            }
                        }

                        reply += "\r\n";
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
