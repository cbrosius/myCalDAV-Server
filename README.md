# My CalDAV Server

A CalDAV-compatible calendar server built with Rust and Axum.

## Features

- **CalDAV Protocol Support**: Compatible with CalDAV clients (DAVx5, iOS, etc.)
- **REST API**: Full REST API for calendar and event management
- **User Management**: Registration, authentication with JWT tokens
- **Calendar Sharing**: Share calendars with other users
- **SQLite Database**: Lightweight, file-based storage

## Quick Start

### Using Docker (Recommended)

1. Open the project in VS Code with Dev Containers extension
2. Reopen in Container when prompted
3. Run the server:

```bash
cargo run
```

### Local Development

1. Install Rust and SQLite
2. Set environment variables (optional):
   ```bash
   export PORT=8080
   export DATABASE_URL=sqlite:./data/calendar.db
   export JWT_SECRET=your-secret-key
   ```
3. Run:
   ```bash
   cargo run
   ```

The server will start on `http://localhost:8080`

## API Endpoints

### Public Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/` | Welcome page |
| GET | `/health` | Health check |
| POST | `/api/auth/register` | Register new user |
| POST | `/api/auth/login` | Login and get JWT token |
| GET | `/.well-known/caldav` | CalDAV discovery |

### Protected Endpoints (Require JWT Token)

Add `Authorization: Bearer <token>` header for all protected endpoints.

#### Users

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/users/{id}` | Get user by ID |

#### Calendars

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/auth/calendars` | Get user's calendars |
| POST | `/api/auth/calendars` | Create new calendar |
| GET | `/api/calendars/{id}` | Get calendar by ID |
| PUT | `/api/auth/calendars/{id}` | Update calendar |
| DELETE | `/api/auth/calendars/{id}` | Delete calendar |

#### Events

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/auth/calendars/{id}/events` | Get calendar events |
| POST | `/api/auth/events` | Create new event |
| GET | `/api/events/{id}` | Get event by ID |
| GET | `/api/auth/events/{id}` | Get event (with auth check) |
| PUT | `/api/auth/events/{id}` | Update event |
| DELETE | `/api/auth/events/{id}` | Delete event |

#### Shares

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/auth/calendars/{id}/shares` | Get calendar shares |
| POST | `/api/auth/calendars/{id}/shares` | Create share |
| DELETE | `/api/auth/shares/{id}` | Delete share |

## Request/Response Examples

### Register User

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "password123"}'
```

Response:
```json
{
  "id": "uuid",
  "email": "user@example.com"
}
```

### Login

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "password123"}'
```

Response:
```json
{
  "token": "jwt-token-here",
  "user": {
    "id": "uuid",
    "email": "user@example.com"
  }
}
```

### Create Calendar

```bash
curl -X POST http://localhost:8080/api/auth/calendars \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"name": "My Calendar", "description": "Personal calendar", "color": "#FF5733", "is_public": false}'
```

### Create Event

```bash
curl -X POST http://localhost:8080/api/auth/events \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "calendar_id": "calendar-uuid",
    "event": {
      "title": "Meeting",
      "description": "Team meeting",
      "location": "Office",
      "start_time": "2024-01-15T10:00:00Z",
      "end_time": "2024-01-15T11:00:00Z",
      "is_all_day": false
    }
  }'
```

## CalDAV Configuration

### DAVx5 (Android)

1. Add new account → CalDAV
2. Base URL: `http://your-server:8080/.well-known/caldav`
3. Username: your email
4. Password: your JWT token (get from login endpoint)

### iOS/macOS

1. Settings → Calendar → Accounts → Add Account
2. Other → CalDAV
3. Server: `http://your-server:8080/.well-known/caldav`
4. User Name: your email
5. Password: your JWT token

## Development

### Devcontainer Usage

This project includes a devcontainer configuration for consistent development environments.

#### Starting the Devcontainer

From the project root directory:

```bash
# Build and start the devcontainer in detached mode
cd .devcontainer && docker-compose up -d --build

# Return to project root
cd ..
```

#### Running Commands in the Devcontainer

Since the host system may not have Rust installed, use `docker exec` to run commands inside the container:

```bash
# Check compilation
docker exec devcontainer-rust-dev-1 cargo check

# Build the project
docker exec devcontainer-rust-dev-1 cargo build

# Run tests
docker exec devcontainer-rust-dev-1 cargo test

# Run the server
docker exec devcontainer-rust-dev-1 cargo run

# Start an interactive shell in the container
docker exec -it devcontainer-rust-dev-1 bash
```

#### Stopping the Devcontainer

```bash
cd .devcontainer && docker-compose down
```

#### VS Code Integration

If using VS Code with the Dev Containers extension:
1. Open the project folder
2. When prompted, click "Reopen in Container"
3. VS Code will automatically build and connect to the devcontainer

The container includes:
- Rust 1.85
- sqlx-cli for database migrations
- cargo-watch for hot-reload during development
- Required system dependencies (SQLite, build tools)

### Project Structure

```
src/
├── main.rs           # Entry point
├── lib.rs            # Application setup and routes
├── config.rs         # Configuration management
├── error.rs          # Error types and handling
├── models.rs         # Data models and DTOs
├── services.rs       # Business logic and database operations
├── middleware.rs     # HTTP middleware (auth, CORS, logging)
├── state.rs          # Application state
├── handlers/         # Request handlers
│   └── auth.rs       # Authentication handlers
├── database/         # Database utilities
│   └── mod.rs        # Database initialization
└── migrations/       # SQL migrations
    └── 001_initial_schema.sql
```

### Running Tests

```bash
cargo test
```

### Building for Production

```bash
cargo build --release
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | 8080 | Server port |
| `DATABASE_URL` | `sqlite:./data/calendar.db?mode=rwc` | SQLite database URL |
| `JWT_SECRET` | `your-secret-key-change-in-production` | JWT signing secret |

## License

MIT License
