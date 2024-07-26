use std::{
    collections::HashMap, io::{BufRead, BufReader, Write}, net::TcpListener, sync::{Arc, RwLock}, thread
};
use std::net::TcpStream;

use chrono::Local;

fn main() {
    // shared info
    let clients : HashMap<String, TcpStream> = HashMap::new();
    let clients = Arc::new(RwLock::new(clients));

    let topic = String::from("unset");
    let topic = Arc::new(RwLock::new(topic));
    ////

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        println!("Connection established!");

        let clients = Arc::clone(&clients);
        let topic = Arc::clone(&topic);

        thread::spawn(move || {
            let mut stream: std::net::TcpStream = stream.unwrap();

            stream.write(b"What is your name?\r\n").unwrap();

            let mut my_name = String::new();

            let mut line_no = 0;
            let reader = BufReader::new(stream.try_clone().unwrap());
            for input in reader.lines() {
                match input {
                    Ok(input) => {
                        line_no += 1;

                        //println!("Received: {}", input);

                        let mut reply = String::new();
                        match input.as_str() {
                            "exit" | "quit" => {
                                stream.write(b"Bye!\r\n").unwrap();
                                break;
                            }
                            "list users" => {
                                let clients = clients.read().unwrap();
                                let names_iter = clients.keys();
                                let names: Vec<String> = names_iter.map(|s| s.to_owned()).collect();
                                let joined_names = names.join(", ");
                                reply = format!("Users online: {}", joined_names);
                            }
                            "get topic" => {
                                let topic = topic.read().unwrap().clone();
                                reply = format!("Current topic: {}", topic);
                            }
                            _ if line_no == 1 => {
                                if input.contains(" ") {
                                    stream.write(b"Your name can't contain spaces!\r\n").unwrap();
                                    break;
                                }
                                my_name = input.clone();

                                let mut clients = clients.write().unwrap();

                                if clients.contains_key(&my_name) {
                                    stream.write(b"Name already in use!\r\n").unwrap();
                                    break;
                                }

                                clients.insert(my_name.clone(), stream.try_clone().unwrap());

                                broadcast(&clients, &format!("{} @ {} joined", time(), my_name), &my_name);

                                reply = format!("Users online: {}, topic: {}!", clients.len(), topic.read().unwrap());
                            }
                            _ => {
                                let tokens = input.split_whitespace().collect::<Vec<_>>();
                                let first = tokens.get(0);
                                let second = tokens.get(1);

                                if first == Some(&"say") && second.is_some() {
                                    let destination = second.unwrap().to_string();
                                    let clients = clients.read().unwrap();
                                    if name_exists(&clients, &destination) {
                                        let text_to_send = tokens.as_slice()[2..].join(" ");
                                        let text_to_send = format!("{} @ {} said: {}", time(), my_name, text_to_send);
                                        say(&clients, &text_to_send, &destination);
                                    } else {
                                        reply = format!("User not found: '{}'", destination);
                                    }
                                }
                                else if first == Some(&"shout") {
                                    let clients = clients.read().unwrap();
                                    let text_to_send = tokens.as_slice()[1..].join(" ");
                                    let text_to_send = format!("{} @ {} shouted: {}", time(), my_name, text_to_send);
                                    broadcast(&clients, &text_to_send, &String::from(""));
                                }
                                else if first == Some(&"set") && second == Some(&"topic") {
                                    let new_topic = tokens.as_slice()[2..].join(" ");
                                    let mut topic = topic.write().unwrap();
                                    *topic = new_topic.clone();
                                    let text = format!("{} @ {} changed topic to: {}", time(), my_name, new_topic);
                                    let clients = clients.read().unwrap();
                                    broadcast(&clients, &text, &String::from(""));
                                } else {
                                    reply = input.to_ascii_uppercase();
                                }
                            }
                        }

                        if reply.len() > 0 {
                            reply += "\r\n";
                            stream.write(reply.as_bytes()).unwrap();
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading from stream: {}", e);
                        break;
                    }
                }
            }

            let mut clients = clients.write().unwrap();
            broadcast(&clients, &format!("{} @ {} left", time(), my_name), &my_name);
            clients.remove(&my_name);

            println!("finishing a thread");
        });
    }
}

fn time() -> String {
    let now = Local::now();
    return now.format("%H:%M:%S").to_string();
}

fn name_exists(clients: &HashMap<String, TcpStream>, name: &String) -> bool {
    return clients.contains_key(name);
}

fn say(clients: &HashMap<String, TcpStream>, text: &String, destination: &String) -> bool {
    let text = text.to_owned() + "\r\n";

    if let Some(mut stream) = clients.get(destination) {
        stream.write_all(text.as_bytes()).unwrap();
        return true
    }
    false
}

fn broadcast(clients: &HashMap<String, TcpStream>, text: &String, avoid_name: &String) {
    let text = text.to_owned() + "\r\n";

    for (current_name, mut client) in clients {
        if avoid_name == current_name {
            continue;
        }

        if let Err(e) = client.write_all(text.as_bytes()) {
            eprintln!("Error broadcasting message: {}", e);
        }
    }
}
