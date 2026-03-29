# MediLink Backend

MediLink is a multi-tenant SaaS platform designed to provide small independent medical clinics with digital tools for managing patients, doctors, and appointments. This repository contains the backend API for the MediLink platform, built in Rust.

## Overview

The MediLink backend exposes a RESTful API consumed by the MediLink frontend. It handles all business logic, authentication, and database interactions. The backend follows a multi-tenant architecture, meaning a single deployed instance serves multiple clinic clients, with strict data isolation enforced at the database level through foreign key constraints and query scoping.

Authentication is implemented using JSON Web Tokens (JWT), with separate token flows for clinic administrators and patients. Passwords are hashed using the Argon2 algorithm before storage.

## Tech Stack

- **Language:** Rust
- **Web Framework:** Axum
- **Database:** PostgreSQL
- **ORM / Query Layer:** sqlx
- **Authentication:** JSON Web Tokens (jsonwebtoken)
- **Password Hashing:** Argon2
- **Configuration:** config + dotenvy

## Key Features

- Clinic registration and authentication
- Patient registration and authentication, scoped to a specific clinic
- Doctor management per clinic
- Appointment booking with double-booking prevention enforced at the database level
- Appointment viewing and cancellation
- Multi-tenant data isolation across routing, application logic, and database layers

## How to Run Locally

### Prerequisites

- Rust (https://rustup.rs)
- PostgreSQL
- sqlx CLI

Install the sqlx CLI:
```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### Setup

1. Clone the repository:
```bash
git clone https://github.com/IoanaGoroneanu/medilink-backend.git
cd medilink-backend
```

2. Create the database:
```bash
sqlx database create --database-url "postgres://localhost/clinic_db"
```

3. Create a configuration file at the root of the project. The application supports both TOML configuration files and environment variables. Create a `config.toml` file:
```toml
host = "127.0.0.1"
port = 8080
database_url = "postgres://localhost/clinic_db"
jwt_secret = "your-secret-key"
```

Alternatively, set the following environment variables:
```bash
export HOST=127.0.0.1
export PORT=8080
export DATABASE_URL=postgres://localhost/clinic_db
export JWT_SECRET=your-secret-key
```

4. Run the database migrations:
```bash
sqlx migrate run --database-url "postgres://localhost/clinic_db"
```

5. Build and run the backend:
```bash
cargo run
```

The server will start on `http://127.0.0.1:8080`.

## API Endpoints

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| POST | `/clinic_signup` | Register a new clinic | None |
| POST | `/clinic_login` | Log in as a clinic | None |
| POST | `/clinic/doctors` | Add a doctor to the clinic | Clinic JWT |
| POST | `/clinics/{slug}/patients/register` | Register a new patient | None |
| POST | `/clinics/{slug}/login` | Log in as a patient | None |
| GET | `/clinics/{slug}/doctors` | List doctors for a clinic | None |
| POST | `/appointments` | Book an appointment | Patient JWT |
| GET | `/appointments` | Get patient appointments | Patient JWT |
| POST | `/appointments/{id}/cancel` | Cancel an appointment | Patient JWT |