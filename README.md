# Transparency Nepal

A civic transparency platform dedicated to tracking infrastructure projects and public spending across Nepal. Our mission is to bridge the gap between citizens and public works by providing real-time data, community feedback, and accountability.

## The Story: How it Started

Born out of a desire to see every rupee of public and donated funds translate into real impact, this project started as a grassroots initiative to digitize the tracking of construction and social projects in Nepal. 

In a landscape where infrastructure projects often lack a central, accessible hub, citizens can struggle to verify progress, see budgets, or report issues. We built Transparency Nepal to bring that data into the light, ensuring that accountability is not just a policy, but a functional reality for every community.

## Core Features

- **Project Tracking**: Visualize infrastructure projects on an interactive map.
- **Real-time Feedback**: Citizens can submit opinions and react to project progress.
- **Accountability**: Track budgets, timelines, and responsible agencies.
- **Secure Access**: Simple, secure phone-based authentication via OTP (SMS/WhatsApp).

## Running Locally

Follow these steps to get the backend up and running on your machine.

### Prerequisites

- **Rust**: [Install Rust](https://rustup.rs/) (Stable)
- **Docker**: For running the database and cache.

### 1. Start Infrastructure
We use **SurrealDB** as our primary database and **DragonflyDB** (Redis-compatible) for ultra-fast caching and OTP management.

```bash
cd apps/backend
docker compose up surrealdb dragonfly -d
```

### 2. Configure Environment
Create a `.env` file in the `apps/backend` directory:

```env
# Infrastructure
SURREAL_URL=127.0.0.1:8000
CACHE_URL=redis://127.0.0.1/
PORT=3000

# Authentication
PASETO_SECRET=your_32_byte_secret_here_for_auth

# Optional: Notification Providers (Mocked in debug mode)
SPARROW_SMS_TOKEN=...
SPARROW_SMS_SENDER=GORKHAS
WHATSAPP_ACCESS_TOKEN=...
WHATSAPP_PHONE_NUMBER_ID=...
```

### 3. Start the Server
```bash
cargo run
```

The API will be available at `http://localhost:3000`. 
> **Note**: In debug mode (default), the OTP is always `123456` and no real messages are sent.

## Project Structure

The codebase is designed to be "Simple, Powerful, and Less Messy" following Clean Architecture principles:

- `src/core/`: The heart of the app—business-level interfaces, state, and unified notifications.
- `src/shared/`: Reusable library components and concrete implementations (adapters) for providers and auth.
- `src/features/`: Vertical slices containing feature-specific logic (Projects, Auth, Contributions, etc.).
- `src/main.rs`: The entry point that wires everything together.

---

*Ensuring every stone laid in Nepal is accounted for.*
