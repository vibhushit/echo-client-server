# Echo Client-Server 

This Rust project consists of four separate Cargo projects that implement various client-server communication patterns using both synchronous (standard library) and asynchronous (Tokio) approaches.

## Project Structure

- `echo-client`: A basic TCP client using the Rust standard library.
- `echo-client-tokio`: An asynchronous TCP client using the Tokio framework.
- `kiren`: A handler that acts as a bridge between the client and the `sirocco` server using asynchronous communication.
- `sirocco`: A single-threaded TCP server using the Rust standard library, with support for simulating delays.

---

### 1. echo-client

A basic TCP client that sends a message to the server and receives a response using Rust's standard library.

**Usage:**
```bash
cargo run --bin echo-client
```
----

### 2. echo-client-tokio

This is a modification of the echo-client, but it will use the Tokio asynchronous runtime.

**Usage:**
```bash
cargo run --bin echo-client-tokio
```
----

### 3. kiren

`kiren` is a handler that listens for requests from a client and forwards them to the `sirocco` server. It will use asynchronous communication (Tokio) to handle multiple requests efficiently. 

**Usage:**
```bash
cargo run --bin kiren
```
----

### 4. sirocco

`sirocco` is a simple, single-threaded web server that echoes back any message it receives. It also supports adding an artificial delay to simulate slow I/O operations.

**Usage:**
```bash
cargo run --bin sirocco <delay-in-milliseconds>
```

**Example:**
```bash
cargo run --bin sirocco 5000
```
----

## How to Run

1. Start the `sirocco` server (with an optional delay).
2. Run the `echo-client` to send a message to the `sirocco` server.

----

## License

This project is licensed under the MIT License - see the LICENSE file for details.
