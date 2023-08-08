#![allow(dead_code)]

use std::{
    io::{/* prelude::*, */ BufReader, BufRead, Write},
    net::{TcpListener, TcpStream},
    error::Error,
    sync::{Arc, Mutex}, fmt::Display,
};

use super::thread_pool::ThreadPool;

#[derive(Debug)]
struct Message {
    author: String,
    content: String,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {}", self.author, self.content)
    }
}

impl Message {
    fn new(author: String, content: String) -> Self {
        Message { author, content }
    }
}

struct Chat {
    msgs: Vec<Message>,
}

impl Chat {
    fn new() -> Self { Chat { msgs: Vec::new() } }
}

pub fn start() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let pool = ThreadPool::new(4).unwrap();

    let chat = Arc::new(Mutex::new(Chat::new()));

    for stream in listener.incoming() {
        let stream = stream?;
        let chat_clone = Arc::clone(&chat);

        pool.execute(|| {
            match handle_connection(stream, chat_clone) {
                Ok(()) => { },
                Err(e) => { println!("[error]: {e}"); },
            };
        });

        println!("{:#?}", chat.lock().unwrap().msgs);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, chat: Arc<Mutex<Chat>>) -> Result<(), Box<dyn Error>> {
    let buf_reader = BufReader::new(&mut stream);
    let request = buf_reader
        .lines()
        .map(|e| e.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>();

    match chat.lock() {
        Ok(mut c) => {
            c.msgs.push(Message::new(
                request
                    .iter()
                    .find(|e| e.contains("user"))
                    .ok_or(format!("author missing"))?
                    .split(':')
                    .collect::<Vec<&str>>()
                    .get(1)
                    .ok_or(format!("author name missing"))?
                    .to_string(),
                request
                    .last()
                    .ok_or(format!("message missing"))?
                    .to_string()
            ));

            let response = c.msgs
                .iter()
                .map(|msg| msg.to_string())
                .collect::<Vec<String>>()
                .join("\n");
            stream.write_all(response.as_bytes())
                .ok().ok_or(format!("couldn't send response back"))?;
        },
        Err(e) => { println!("[error]: {e}"); },
    }

    Ok(())
}
