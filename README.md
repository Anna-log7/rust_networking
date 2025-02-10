# Basic HTTP server with Rust
This is a basic server listening on the loopback address, serving the routes `/`, `/sleep`, and a 404 route.

## Features
### Threading
The server is initialized with a `ThreadPool` and will spawn threads to handle requests until the max thread limit is reached

### Rate limiting
A `Bucket` is created using the Token Bucket algorithm to limit requests to a defined rate
