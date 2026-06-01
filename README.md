# edu

A portfolio Rust project implementing a minimal HTTP server and learning backend logic for tracking subscriptions and Solana integration.

## Overview

This repository is a personal portfolio project built to explore how a basic server works in Rust and to connect Solana tracking concepts to a local web app.

The server is designed to:
- accept HTTP requests over a simple TCP listener
- serve static frontend files from the `static/` folder
- expose API endpoints for simple question data and session handling
- use SQLite via `rusqlite` for local storage
- demonstrate a foundation for future Solana subscription tracking features

## Project Goals

- Learn how a low-level Rust HTTP server handles connections, request parsing, and routing.
- Build a small backend with database persistence and session management.
- Create a frontend/backend portfolio example with a clear architecture.
- Prepare the codebase for future Solana integration to monitor subscriptions or wallet activity.

## Tech Stack

- Rust 2024 edition
- `rusqlite` for SQLite database access
- `serde` / `serde_json` for JSON serialization
- `uuid` for basic session token generation
- Custom HTTP parser and router

## Architecture

- `src/main.rs` - entrypoint, TCP listener, thread pool, shared database, and session state
- `src/http.rs` - HTTP request and response parsing
- `src/router.rs` - request routing, static file serving, and API handling
- `src/db.rs` - SQLite database wrapper, question storage, and query logic
- `src/thread_pool.rs` - simple thread pool to handle concurrent connections
- `static/` - static frontend assets served by the server

## API Endpoints

- `GET /api/subjects` - returns a JSON list of subjects from the database
- `GET /api/questions/:subject` - returns a random set of questions for the given subject and stores them in a session
- `GET /static/...` - serves static files such as HTML, CSS, JavaScript, and icons

## Running the Project

1. Install Rust if needed: https://www.rust-lang.org/tools/install
2. Build and run the server:

```bash
cargo run
```

3. Open the browser at:

```text
http://localhost:7070
```

The server listens on port `7070` by default and creates a local SQLite file named `questions.db`.

## Notes

- This project is intentionally simple and focused on learning server internals rather than production-ready features.
- Solana-specific logic is not yet implemented in the current version, but the structure is ready for adding subscription tracking or blockchain event polling.

## Next Steps

- add a Solana client or RPC layer to track subscriptions and wallet updates
- implement endpoint(s) for real-time subscription state
- improve session handling and request validation
- expand the frontend to display Solana subscription data

## License

This repository is intended for personal portfolio and learning purposes.
