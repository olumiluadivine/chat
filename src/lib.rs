use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod util;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Client {
    Join {
        chat_name: Arc<String>,
    },
    Post {
        chat_name: Arc<String>,
        message: Arc<String>,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Server {
    Message {
        chat_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}

#[test]
fn test_client() {
    use std::sync::Arc;

    let client = Client::Post {
        chat_name: Arc::new(String::from("value")),
        message: (Arc::new(String::from("value"))),
    };

    let json = serde_json::to_string(&client).unwrap();
    println!("{}", json);
    assert_eq!(json, r#"{"Post":{"chat_name":"value","message":"value"}}"#);
    assert_eq!(serde_json::from_str::<Client>(&json).unwrap(), client);
}
