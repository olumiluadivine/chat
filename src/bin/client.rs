use async_std::prelude::*;
use async_std::{io, net, task};
use chat::{
    util::{self, ChatResult},
    Client, Server,
};
use std::sync::Arc;

async fn send(mut send: net::TcpStream) -> ChatResult<()> {
    println!("Options :\njoin CHAT\npost CHAT MESSAGE\nType CTRL-D(Unix) or CTRL-Z*(Windows) to close the connection");

    let mut option: io::Lines<io::BufReader<io::Stdin>> = io::BufReader::new(io::stdin()).lines();

    while let Some(option_result) = option.next().await {
        let opt: String = option_result?;
        let req: Client = match parse_input(&opt) {
            Some(req) => req,
            None => continue,
        };
        util::send_json(&mut send, &req).await?;
        send.flush().await?;
    }
    Ok(())
}

async fn messages(server: net::TcpStream) -> ChatResult<()> {
    let buf: io::BufReader<net::TcpStream> = io::BufReader::new(server);
    let mut stream = util::receive(buf);

    while let Some(msg) = stream.next().await {
        match msg? {
            Server::Message { chat_name, message } => {
                println!("Chat Name: {}\n Message: {}\n", chat_name, message);
            }
            Server::Error(message) => {
                println!("Error received: {}", message);
            }
        }
    }
    Ok(())
}

fn main() -> ChatResult<()> {
    let addr = std::env::args().nth(1).expect("Address:PORT");

    task::block_on(async {
        let socket: net::TcpStream = net::TcpStream::connect(addr).await?;
        socket.set_nodelay(true)?;
        let send = send(socket.clone());
        let replies = messages(socket);
        replies.race(send).await?;
        Ok(())
    })
}

// fn get_value(mut input: &str) -> Option<&str, &str> {
//     input = input.trim_start();
//     if input.is_empty() {
//         return None;
//     }

//     match input.find(char::is_whitespace) {
//         Some(whitespace) => Some((&input[0.. whitespace], &input[whitespace..])),
//         None => Some((input, "")),
//     }
// }
fn get_value(mut input: &str) -> Option<(&str, &str)> {
    input = input.trim_start();
    if input.is_empty() {
        return None;
    }

    match input.find(char::is_whitespace) {
        Some(whitespace) => Some((&input[0..whitespace], &input[whitespace..])),
        None => Some((input, "")),
    }
}

fn parse_input(line: &str) -> Option<Client> {
    let (input, remainder) = get_value(line)?;
    if input == "join" {
        let (chat, remainder) = get_value(remainder)?;
        if !remainder.trim_start().is_empty() {
            return None;
        }
        return Some(Client::Join {
            chat_name: Arc::new(chat.to_string()),
        });
    } else if input == "post" {
        let (chat, remainder) = get_value(remainder)?;
        let m = remainder.trim_start().to_string();
        return Some(Client::Post {
            chat_name: Arc::new(chat.to_string()),
            message: Arc::new(m),
        });
    } else {
        println!("Unrecognizedd input: {:?}", line);
        return None;
    }
}
