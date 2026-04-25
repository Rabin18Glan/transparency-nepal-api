# Transparency in the Land of Gorkhas — Backend

High-performance Rust API for a civic transparency platform tracking infrastructure projects across Nepal.

## Stack

| Layer | Technology | Why |
|---|---|---|
| HTTP | [Axum](https://github.com/tokio-rs/axum) | Async, zero-cost abstractions, type-safe extractors |
| Database | [SurrealDB](https://surrealdb.com) | Multi-model, geospatial queries, graph relationships |
| Cache | [DragonflyDB](https://www.dragonflydb.io) | 25× faster Redis-compatible cache for OTP storage |
| Runtime | Tokio | Industry-standard async runtime |
| SMS | [Sparrow SMS](https://sparrowsms.com) | Nepal-native SMS gateway |
| WhatsApp | [Meta Cloud API](https://developers.facebook.com/docs/whatsapp) | Official WhatsApp messaging |

## Architecture

Vertical Slice Architecture — each feature is self-contained with its own `model`, `repo`, `service`, and `controller`.

```
src/
├── config/           # App-wide infrastructure (env, db, cache, providers)
├── common/           # Shared utilities (OTP generation)
├── features/
│   ├── auth/         # OTP request & verify
│   └── projects/     # Project CRUD & map feed
├── error.rs          # Unified error type → HTTP status mapping
├── state.rs          # Shared application state (Arc<AppState>)
└── main.rs
```

## Setup

### Prerequisites

- Rust (stable)
- Docker

### 1. Start Infrastructure

```bash
docker compose up surrealdb dragonfly -d
```

### 2. Configure Environment

Copy `.env` and fill in your credentials:

```env
SURREAL_URL=127.0.0.1:8000
PORT=3000
CACHE_URL=redis://127.0.0.1/

# Sparrow SMS (https://sparrowsms.com)
SPARROW_SMS_TOKEN=your_token_here
SPARROW_SMS_SENDER=GORKHAS

# Meta WhatsApp Cloud API (https://developers.facebook.com)
WHATSAPP_ACCESS_TOKEN=your_access_token_here
WHATSAPP_PHONE_NUMBER_ID=your_phone_number_id_here
```

### 3. Run

```bash
cargo run
```

> **Debug mode**: OTP is always `123456` and no real SMS/WhatsApp messages are dispatched.
> Build with `cargo build --release` to enable real providers.

## API Reference

### Auth

#### `POST /api/auth/otp/request`

Request an OTP via SMS or WhatsApp.

```json
{
  "phone_number": "9841234567",
  "channel": "sms"        // "sms" (default) | "whatsapp"
}
```

**Response**
```json
{
  "message": "OTP successfully sent",
  "expires_in_seconds": 300
}
```

---

#### `POST /api/auth/otp/verify`

Verify a received OTP.

```json
{
  "phone_number": "9841234567",
  "otp": "123456"
}
```

**Response**
```json
{
  "verified": true,
  "message": "OTP verified successfully"
}
```

---

### Projects

#### `POST /api/projects`

Create a new infrastructure project.

```json
{
  "title": "Gorkha Community Hospital",
  "description": "Funding an additional wing for the pediatric ward.",
  "expected_budget": 50000.0,
  "latitude": 28.0006,
  "longitude": 84.6254
}
```

---

#### `GET /api/projects`

Retrieve all projects for map rendering.

```json
[
  {
    "title": "Gorkha Community Hospital",
    "status": "planning",
    "latitude": 28.0006,
    "longitude": 84.6254,
    ...
  }
]
```

## Docker (Full Stack)

To run the entire stack including the API in Docker:

```bash
docker compose up
```

> Requires a production build. See `Dockerfile` for the multi-stage build configuration.

## Error Format

All errors return a consistent JSON body:

```json
{
  "error": "Human-readable description"
}
```

| Status | Meaning |
|---|---|
| `400` | Validation error |
| `401` | Unauthorized (wrong OTP) |
| `404` | Not found (OTP expired) |
| `500` | Internal server error |
