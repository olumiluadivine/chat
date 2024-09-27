# Async Chat Application

This is an asynchronous chat application implemented in Rust using the `async-std` library. The application supports joining chat rooms and posting messages to them.

## Features

- Join chat rooms
- Post messages to chat rooms
- Asynchronous communication using `async-std`
- JSON-based message format

## Project Structure

- `main.rs`: The entry point of the application. It sets up the TCP connection and handles the main logic for sending and receiving messages.
- `client.rs`: Defines the `Client` enum and related logic for client-side operations.
- `server.rs`: Defines the `Server` enum and related logic for server-side operations.
- `util.rs`: Contains utility functions for sending and receiving JSON messages over TCP.

## Dependencies

- `async-std`: Provides asynchronous versions of standard library components.
- `serde`: Framework for serializing and deserializing Rust data structures efficiently and generically.
- `serde_json`: JSON serialization and deserialization.

## Usage

### Running the Server

To run the server, use the following command:

```sh
cargo run --release --bin server -- localhost:{port}
```

### Running the Client

To run the client, use the following command:

```sh
cargo run --release --bin client -- localhost:{port}
```

## Commands

Once the client is running, you can use the following commands:

- `join <CHAT_NAME>`: Join a chat room.
- `post <CHAT_NAME> <MESSAGE>`: Post a message to a chat room.

## Example
```sh
cargo run --release --bin client -- localhost:8080
```
Options:
```txt
join CHAT
post CHAT MESSAGE
Type CTRL-D(Unix) or CTRL-Z*(Windows) to close the connection
```

## Join a Chat Room
```sh
join general
```

## Post a Message
```sh
post general Hello, world!
```

## Contributing
Contributions are welcome! Please open an issue or submit a pull request on GitHub.
