# 🦀 Day 3: Rust Reverse Proxy - Error Handling & Async Programming

## Project Overview

**Rust Reverse Proxy** is a lightweight HTTP reverse proxy server built with Axum and Tokio. This server sits between clients and an upstream service, forwarding requests and responses while providing detailed logging. The project demonstrates several advanced Rust concepts through a practical, real-world networking application.

![Divider](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/rainbow.png)

## 🔍 How It Works

This reverse proxy:

1. **Receives incoming HTTP requests** on port 8080
2. **Logs detailed request information** including method, URI, and headers
3. **Forwards the request** to an upstream server (httpbin.org)
4. **Captures the response** from the upstream server
5. **Returns the response** to the original client

The project illustrates several key Rust concepts:

* **Modular organization** with separate files for core functionality
* **Custom error handling** with the `thiserror` crate
* **Asynchronous programming** with Tokio and async/await
* **HTTP routing and handling** with the Axum framework

![Divider](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/solar.png)

## 🧩 Understanding Error Handling in Rust

### Custom Error Types with `thiserror`

One of the standout features of this project is its approach to error handling. Rust encourages explicit error handling through the `Result<T, E>` type, and this project takes it a step further with custom error types:

```rust
#[derive(Debug, Error)]
pub enum ProxyError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to convert body: {0}")]
    BodyConversionError(String),
}
```

This approach provides several benefits:

* **Type safety**: Errors are typed, making handling predictable
* **Rich context**: Each error variant can carry specific information
* **Clear messages**: The `#[error]` attribute provides human-readable descriptions
* **Automatic conversion**: The `#[from]` attribute enables `?` operator usage

### Converting Errors to HTTP Responses

The project also demonstrates how to convert these custom errors into HTTP responses:

```rust
impl IntoResponse for ProxyError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            ProxyError::BodyConversionError(e) => {
                (StatusCode::BAD_REQUEST, format!("Body error: {}", e))
            }
            ProxyError::RequestError(e) => {
                (StatusCode::BAD_GATEWAY, format!("Upstream error: {}", e))
            }
        };
        (status, msg).into_response()
    }
}
```

This pattern allows our error handling to be seamlessly integrated with Axum's response system.

![Divider](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/fire.png)

## 🔄 Asynchronous Programming with Tokio

### Understanding Async/Await

This project showcases Rust's async/await syntax, which enables non-blocking I/O operations:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Async code here
}
```

The `async` keyword marks functions that can be paused and resumed, while `await` is used to yield control until an async operation completes.

### Benefits of Async in a Proxy Server

In our reverse proxy, async programming provides:

* **High concurrency**: Handle many connections without spawning OS threads
* **Efficient resource use**: I/O-bound operations don't block the thread
* **Scalability**: The server can handle many simultaneous connections
* **Readability**: Sequential-looking code that's actually non-blocking

## 💻 The Proxy Handler

The core of our application is the proxy handler function:

```rust
pub async fn proxy_handler(req: Request<Body>) -> Result<impl IntoResponse, ProxyError> {
    // Extract information from request
    // Forward request to upstream server
    // Process the response
    // Return response to client
}
```

This function:
1. Takes an incoming HTTP request
2. Extracts and logs important information
3. Forwards the request to the upstream server
4. Processes the response
5. Returns the response to the client

![Divider](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/aqua.png)

## 🧠 Key Concepts Demonstrated

### 1. Modular Code Organization

The project is organized into multiple modules:
- `main.rs`: Entry point and server setup
- `proxy.rs`: Core proxy handling logic
- `config.rs`: Configuration constants
- `error.rs`: Custom error types and handling

### 2. Error Handling

- Custom error enum with the `thiserror` crate
- Proper error propagation with the `?` operator
- Converting errors to appropriate HTTP responses

### 3. Asynchronous Programming

- Using `async`/`await` for non-blocking I/O
- Tokio runtime for executing async tasks
- Handling HTTP requests and responses asynchronously

### 4. HTTP Request/Response Handling

- Working with HTTP headers, methods, and bodies
- Proper forwarding of request details
- Maintaining HTTP semantics between client and server

## 🚀 Running the Project

To run this reverse proxy:

1. Ensure you have Rust installed
2. Navigate to the project directory
3. Run with cargo:

```bash
cargo run
```

4. The proxy will start on http://127.0.0.1:8080
5. Send requests to this address, and they'll be forwarded to httpbin.org

You can test it with curl:

```bash
curl http://localhost:8080/get
```

## 🔍 Further Exploration

Possible enhancements to this project:
- Add request/response transformation capabilities
- Implement load balancing across multiple upstream servers
- Add authentication and authorization
- Implement rate limiting
- Add caching for improved performance

![Divider](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/rainbow.png)

## 📚 What We've Learned

- How to build a functional HTTP reverse proxy in Rust
- Creating custom error types with rich context
- Writing asynchronous code with Tokio and async/await
- Working with HTTP requests and responses
- Organizing code into logical modules

This project demonstrates how Rust's strong type system, error handling, and async capabilities make it an excellent choice for networking applications where performance and reliability are critical.