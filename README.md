# Insane Web Service

A modern web service built with Rust, Axum, and Tokio.

[中文文档](README.zh-CN.md)

## Features

- 🚀 High-performance web framework based on Axum
- ⚡ Tokio async runtime support
- 📊 Built-in visit counter (using atomic counter)
- 🔍 Health check endpoint
- 📝 JSON message handling
- 📈 Request logging

## Tech Stack

- Rust 2021 Edition
- Axum 0.7.3 (Web Framework)
- Tokio 1.35.1 (Async Runtime)
- Tower 0.4.13 (HTTP Service Components)
- Serde (JSON Serialization/Deserialization)
- Tracing (Logging System)

## API Endpoints

### Health Check
```
GET /health
```
Returns the service health status and version information.

Example response:
```json
{
    "status": "healthy",
    "version": "0.1.0"
}
```

### Visit Counter
```
GET /visit
```
Returns the current visit count.

Example response:
```
Visit count: 1
```

### Message Echo
```
POST /echo
Content-Type: application/json

{
    "content": "Hello, Axum!"
}
```
Returns the received message content as-is.

## Quick Start

1. Clone the project
```bash
git clone <repository-url>
cd insane
```

2. Build the project
```bash
cargo build
```

3. Run the service
```bash
cargo run
```
The service will start at http://127.0.0.1:3000

## API Testing

Test the endpoints using curl:

```bash
# Health check
curl http://localhost:3000/health

# Visit counter
curl http://localhost:3000/visit

# Message echo
curl -X POST -H "Content-Type: application/json" \
     -d '{"content":"Hello, Axum!"}' \
     http://localhost:3000/echo
```

## Project Structure

```
insane/
├── src/
│   └── main.rs      # Main program entry
├── Cargo.toml       # Project dependencies
└── README.md        # Project documentation
```

## Development Roadmap

- [ ] Add database integration
- [ ] Implement user authentication
- [ ] Add more API endpoints
- [ ] Add static file serving
- [ ] Implement WebSocket support

## Contributing

Issues and Pull Requests are welcome!

## License

MIT License
