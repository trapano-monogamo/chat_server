use std::{
    net::TcpStream, io::{stdin, Write, stdout, Read},
};

// use crossterm::terminal;

fn main() {
    let mut name = String::new();

    print!("user name: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    let mut stream: TcpStream;
    let mut msg: Vec<String>;
    let mut buffer = String::new();

    let mut response = String::new();

    loop {
        stream = TcpStream::connect("127.0.0.1:8080").unwrap();

        msg = vec![
            "#header".to_string(),
            format!("user:{}", name),
            "#body".to_string(),
        ];

        buffer.clear();
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut buffer).unwrap();
        msg.push(buffer.clone());
        msg.push("\n".to_string());

        match stream.write_all(msg.join("\n").as_bytes()) {
            Ok(_) => { },
            Err(e) => { println!("[error]: {e}"); },
        };

        response.clear();
        stream.read_to_string(&mut response).unwrap();
        println!("{response}");
    }
}
