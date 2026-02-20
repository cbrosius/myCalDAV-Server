# Calendar Server Implementation Task List

## Phase 1: Project Foundation & Setup
- [x] create devcontainer for rust-development and continue inside docker container
- [x] Initialize Rust project with Cargo
- [x] Set up project structure (src/, docs/, plans/, etc.)
- [x] Create basic Cargo.toml with dependencies
- [x] Set up Git repository and initial commit
- [x] Create .gitignore for Rust/Docker
- [x] Set up Docker configuration (Dockerfile, docker-compose.yml)

## Phase 2: Database & Data Models
- [x] Design database schema for users, calendars, events, shares
- [x] Set up SQLite database with sqlx
- [x] Create database migrations
- [x] Implement data models (User, Calendar, Event, Share)
- [x] Add database connection pooling
- [x] Create repository layer for data access

## Phase 3: Core Backend API
- [x] Set up Axum web framework
- [x] Implement authentication middleware (JWT sessions)
- [x] Create user management endpoints (CRUD)
- [x] Create calendar management endpoints
- [x] Create event management endpoints with ETag support
- [x] Implement share management endpoints
- [x] Add error handling and validation
- [ ] Create OpenAPI documentation

## Phase 4: CalDAV Server Integration
- [x] Implement CalDAV discovery endpoint (/.well-known/caldav)
- [x] Implement CalDAV PROPFIND for calendar discovery
- [x] Implement CalDAV REPORT for calendar queries
- [x] Implement CalDAV GET for calendar/event data
- [x] Implement CalDAV PUT for creating/updating events
- [x] Implement CalDAV DELETE for removing events
- [x] Add CalDAV authentication (Basic Auth support)
- [ ] Test CalDAV compatibility with DAVx5 and iOS
- [x] Add MKCOL support for creating calendars via CalDAV

## Phase 5: Web GUI Development
- [x] Design frontend architecture (server-rendered with Askama templates)
- [x] Create admin web interface
- [x] Implement user management UI
- [x] Create calendar management UI
- [x] Build event editing interface
- [x] Add sharing features UI
- [ ] Implement QR code generation UI
- [x] Add responsive design and styling

## Phase 6: Security & Authentication
- [x] Implement password hashing (bcrypt)
- [ ] Add HTTPS enforcement (reverse proxy configuration)
- [ ] Create role-based access control (admin/user)
- [x] Implement session management (JWT)
- [ ] Add CSRF protection
- [ ] Create password reset functionality

## Phase 7: Advanced Features
- [ ] Implement ICS export/import functionality
- [ ] Add recurring events support (RRULE)
- [ ] Create public calendar access
- [ ] Implement QR code generation API
- [x] Add conflict resolution with ETags
- [ ] Create search functionality for events

## Phase 8: Testing & Quality Assurance
- [ ] Write unit tests for all components
- [ ] Create integration tests with Docker
- [ ] Add end-to-end tests for web GUI
- [ ] Implement test coverage reporting
- [ ] Set up CI/CD pipeline (Gitea Actions)
- [ ] Add performance testing

## Phase 9: Deployment & Operations
- [ ] Create production configuration
- [x] Set up logging (structured JSON logging via tracing)
- [x] Add health check endpoints
- [ ] Create backup procedures
- [ ] Set up monitoring and alerting
- [ ] Document deployment procedures

## Phase 10: Documentation & Polish
- [ ] Complete API documentation
- [ ] Create user guides and admin manual
- [ ] Add deployment guides
- [ ] Create troubleshooting documentation
- [ ] Add contribution guidelines
- [ ] Prepare for initial release

## Progress Tracking
- [x] Phase 1: Project Foundation & Setup - 100%
- [x] Phase 2: Database & Data Models - 100%
- [x] Phase 3: Core Backend API - 90%
- [x] Phase 4: CalDAV Server Integration - 90%
- [x] Phase 5: Web GUI Development - 85%
- [-] Phase 6: Security & Authentication - 40%
- [ ] Phase 7: Advanced Features - 10%
- [ ] Phase 8: Testing & Quality Assurance - 0%
- [-] Phase 9: Deployment & Operations - 30%
- [ ] Phase 10: Documentation & Polish - 0%

## Notes
- Core CalDAV server functionality is implemented
- Authentication with JWT is working
- Basic Auth support for CalDAV clients (DAVx5, iOS) has been added
- MKCOL support for creating calendars via CalDAV has been added
- CRUD operations for users, calendars, events, and shares are complete
- Web GUI has been implemented with Askama templates
- Next steps: Test with real CalDAV clients (DAVx5, iOS)
- Remaining: QR code generation UI, testing, documentation

## Current Implementation Status

### Completed Features:
1. **User Management**
   - User registration (`POST /api/auth/register`)
   - User login (`POST /api/auth/login`)
   - User retrieval (`GET /api/users/{id}`)

2. **Calendar Management**
   - Create calendar (`POST /api/auth/calendars`)
   - Get user calendars (`GET /api/auth/calendars`)
   - Get calendar by ID (`GET /api/calendars/{id}`)
   - Update calendar (`PUT /api/auth/calendars/{id}`)
   - Delete calendar (`DELETE /api/auth/calendars/{id}`)

3. **Event Management**
   - Create event (`POST /api/auth/events`)
   - Get event by ID (`GET /api/auth/events/{id}`)
   - Get calendar events (`GET /api/auth/calendars/{id}/events`)
   - Update event (`PUT /api/auth/events/{id}`)
   - Delete event (`DELETE /api/auth/events/{id}`)

4. **Share Management**
   - Create share (`POST /api/auth/calendars/{id}/shares`)
   - Get calendar shares (`GET /api/auth/calendars/{id}/shares`)
   - Delete share (`DELETE /api/auth/shares/{id}`)

5. **CalDAV Protocol**
   - Discovery endpoint (`GET /.well-known/caldav`)
   - Calendar discovery (PROPFIND)
   - Event queries (REPORT)
   - Event retrieval (GET)
   - Event creation/update (PUT)
   - Event deletion (DELETE)

### API Endpoints Summary:
| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| GET | / | Root page | No |
| GET | /health | Health check | No |
| POST | /api/auth/register | Register new user | No |
| POST | /api/auth/login | Login | No |
| GET | /api/users/{id} | Get user by ID | Yes |
| GET | /api/calendars/{id} | Get calendar by ID | No |
| GET | /api/auth/calendars | Get user calendars | Yes |
| POST | /api/auth/calendars | Create calendar | Yes |
| PUT | /api/auth/calendars/{id} | Update calendar | Yes |
| DELETE | /api/auth/calendars/{id} | Delete calendar | Yes |
| GET | /api/auth/calendars/{id}/events | Get events | Yes |
| GET | /api/events/{id} | Get event by ID | No |
| POST | /api/auth/events | Create event | Yes |
| GET | /api/auth/events/{id} | Get event | Yes |
| PUT | /api/auth/events/{id} | Update event | Yes |
| DELETE | /api/auth/events/{id} | Delete event | Yes |
| GET | /api/auth/calendars/{id}/shares | Get shares | Yes |
| POST | /api/auth/calendars/{id}/shares | Create share | Yes |
| DELETE | /api/auth/shares/{id} | Delete share | Yes |
| GET | /.well-known/caldav | CalDAV discovery | No |
