# music_platform

## Introduction

**music_platform** is a backend platform built with Rust, using the Axum framework, providing APIs for an online music streaming application. The project supports user management, playlists, music file upload/retrieval, listening history, favorites, JWT authentication, and various utilities.

## Architecture Overview

- **Language:** Rust (edition 2021)
- **Framework:** Axum (supports HTTP/2, WebSocket, multipart, TLS)
- **ORM:** SQLx (PostgreSQL)
- **Authentication:** JWT, Argon2 (password hashing)
- **Utilities:** Symphonia (audio file processing), dotenv, tracing, validator, regex, uuid, etc.

## Main Modules

- `auth`: Authentication, login, registration, token refresh.
- `users`: User information management.
- `upload`: Upload music files, store metadata.
- `getfile`: Retrieve music files, support streaming.
- `playlists`: Manage personal playlists.
- `favorites`: Manage favorite tracks.
- `history`: Store listening history.
- `databases`: Data access layer for the above entities.
- `utils`: Utilities for password hashing, token generation, etc.

## Main Endpoints

All endpoints are under `/api`:

- `POST /api/auth/login` - Login
- `POST /api/auth/register` - Register
- `POST /api/auth/refresh` - Refresh token
- `GET/PUT /api/users` - Get/update user information
- `POST /api/upload` - Upload music file (max 5MB)
- `GET /api/get/{file_id}` - Retrieve music file
- `GET/POST/DELETE /api/playlist` - Manage playlists
- `GET/POST/DELETE /api/favorite` - Manage favorite tracks
- `GET/POST /api/history` - Listening history

## Technologies & Libraries Used

- **axum, axum-server, axum-extra:** Build RESTful APIs, manage routes, middleware, CORS, cookies, TLS.
- **sqlx:** Connect and interact with PostgreSQL.
- **argon2:** Secure password hashing.
- **jsonwebtoken:** Generate and verify JWT.
- **symphonia:** Process audio file formats (mp3, flac, wav, ogg, aac).
- **tokio:** Asynchronous processing.
- **validator, serde, serde_json:** Data validation and serialization.
- **dotenv:** Manage environment variables.

## Getting Started

1. Install Rust and Cargo.
2. Install PostgreSQL, create a database, and configure environment variables in the `.env` file.
3. Run the following commands:
   ```bash
   cd back_end
   cargo run
   ```
4. The server will run at `http://localhost:8000`.

## Contribution

All contributions are welcome! Please create an issue or pull request.
