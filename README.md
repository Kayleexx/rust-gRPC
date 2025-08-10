# Rust gRPC Streaming

This project is a **gRPC server and client in Rust** using the [`tonic`](https://crates.io/crates/tonic) crate.
It demonstrates **server-side streaming** — where the server sends multiple responses for a single client request.

## Features

* gRPC server with server-side streaming
* Simple Rust client to connect and receive streamed messages
* `proto` definitions compiled at build time
* Easy to run and test locally

## How it works

1. **Client** sends a name to the server.
2. **Server** sends back 5 greeting messages, one every 0.5 seconds.
3. Messages appear in real time on the client.

## Project structure

```
rust-grpc/
  build.rs
  Cargo.toml
  proto/hello.proto
  src/
    main.rs   // server
    client.rs // client
```

## Prerequisites

* [Rust](https://www.rust-lang.org/) (latest stable)
* [Protobuf Compiler (`protoc`)](https://grpc.io/docs/protoc-installation/)

On Ubuntu/Debian:

```bash
sudo apt update
sudo apt install protobuf-compiler
```

## Running

Open **two terminals** in the project root:

**Terminal 1** (start server):

```bash
cargo run --bin rust-grpc
```

**Terminal 2** (start client):

```bash
cargo run --bin client
```

You’ll see multiple greeting messages stream in real-time.

## Example output

```
Hello Alice! Message #1
Hello Alice! Message #2
Hello Alice! Message #3
Hello Alice! Message #4
Hello Alice! Message #5
```

