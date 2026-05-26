---

## Features

- Shorten long URLs into small shareable links
- Redirect short links to the original destination
- JSON API
- In-memory storage
- Lightweight and fast

---

## Built With

- Rust
- Axum
- Tokio
- Serde
- NanoID

---

## How It Works

The application exposes two main routes:

---

### `GET /`

Health check endpoint.

Example:

```bash
http://localhost:3000/
```

Returns:

```txt
Rust URL Shortener is running
```

---

### `POST /shorten`

Creates a shortened URL.

Example request:

```bash
curl -X POST http://localhost:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url":"https://example.com"}'
```

Response:

```json
{
  "code": "abc123",
  "short_url": "http://localhost:3000/abc123",
  "original_url": "https://example.com"
}
```

---

### `GET /{code}`

Redirects to the original URL.

Example:

```bash
http://localhost:3000/abc123
```

Redirects to:

```bash
https://example.com
```

---

## Running Locally

Clone the project:

```bash
git clone <repo-url>
cd rust-url-shortener
```

Run:

```bash
cargo run
```

Server starts on:

```bash
http://localhost:3000
```

---

## Project Structure

```bash
src/
 └── main.rs
Cargo.toml
README.md
```

---

## Current Limitations

Currently URLs are stored only in memory using a `HashMap`.

That means:

- links exist only while the server is running
- restarting the server clears all shortened URLs

---

## Possible Improvements

Future ideas:

- SQLite persistence
- Click analytics / visit counter
- Expiration dates for links
- Custom aliases
- Frontend UI
- Docker deployment
- Rate limiting
- QR code generation

---

## Why I Built This

I built this project to get hands-on experience with backend development in Rust and to better understand:

- HTTP routing
- JSON APIs
- application state
- request/response handling
- async programming with Tokio

It also served as a fun way to explore Rust outside of game development and scripting.

---
