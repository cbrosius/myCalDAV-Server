# Calendar Server Requirements Document

## 1. Overview

- **System name:** Open Calendar Server ("OpenCal")  
- **Purpose:**  
  Centralized server for user‑specific and public calendars, accessible via CalDAV/ICS as well as a web API.  
- **Target system:**  
  - Multiple users and public calendars  
  - Access from Android/iOS clients, a web GUI, and external services  
- **Programming language:** Rust  
- **Hosting:**  
  - containerization (Docker, Docker-Compose)  
  - Persistence via database (SQLite)

---

## 2. Non‑functional Requirements

- **Platform & Language:**  
  - Backend implemented in **Rust**, asynchronous (e.g., using Tokio).  
- **API Interface:**  
  - Backend exposes an HTTP‑based API for all operations (REST‑like).  
  - API documentation (e.g., OpenAPI) shall exist.  
- **Security:**  
  - Enforcement of HTTPS (e.g., via Reverse Proxy).  
  - Passwords stored with modern hashing (e.g., Argon2 or BCrypt).  
  - Sessions/JWT for web GUI login.  
- **Scalability & Performance:**  
  - Support for multiple concurrent editors (ETag‑based conflict handling).  
- **Source Code Management:**  
  - Source code hosted in a **Github repository**
  - CI/CD pipeline for automated testing and deployment.
  - Unit tests (e.g., with `cargo test`) and integration tests (e.g., with `cargo test` + Docker).
- **Hosting:**  
  - containerization (Docker, Docker-Compose)
  - Persistence via database (SQLite)
- **Monitoring & Logging:**  
  - Structured logging (e.g., JSON) for debugging and monitoring.

---

## 3. Functional Requirements – Core

### 3.1 Data Models

- **Calendar:**  
  - Owner user (User).  
  - Name, description, color, visibility (public/private).  
  - Each calendar has its own iCalendar structure with events.  

- **User:**  
  - Username, optional email, hashed password.  
  - Roles/permissions:  
    - `admin` (full management)  
    - `user` (use own calendars and shares)  

- **Event:**  
  - UID, start/duration or start/end, timezone‑aware.  
  - Title, description, location, category, status (Tentative/Confirmed/Cancelled).  
  - Support for recurring events (RRULE) according to iCalendar.  

- **Share:**  
  - Calendar ID, optional target user (null if anonymous link), permissions (read/write).  
  - Optional: **anonymous link** with token (e.g., ics URL or web view).

---

### 3.2 Backend Features

- **Multi‑calendar support:**  
  - Server manages multiple user calendars and public calendars at the same time.  

- **CalDAV Compatibility:**  
  - Provide CalDAV endpoints for use by Android (e.g., DAVx⁵) and iOS.  
  - Support key CalDAV operations (CREATE, UPDATE, DELETE, PROPFIND, REPORT).  

- **ICS Export/Import:**  
  - Each calendar can be exported as an ICS file for clients that only support ICS.  

- **Simultaneous Editing:**  
  - Optimistic locking and ETag‑based updates:  
    - PUT/UPDATE must include `If‑Match`/`If‑None‑Match` to prevent overwriting obsolete events.  

- **QR Code Integration:**  
  - API endpoint that generates a QR code for a specific calendar or event.  
  - Scanning the QR code leads to a read‑only access (public URL or login link depending on configuration).  

- **Public Calendars:**  
  - Calendars can be marked as "public" and readable without login.  
  - Public calendars must be listable (e.g., via web GUI or API).

---

## 4. Admin Web GUI

The admin web GUI shall be served via HTTP, either as a single‑page application or server‑rendered frontend (e.g., Axum/Tide + Leptos/Yew or React/Vue).

### 4.1 Admin Features

- **User Management:**  
  - Create, enable/disable, delete users.  
  - Overview table with username, status, number of calendars.  

- **Calendar Management:**  
  - List of all calendars (user, name, type public/private).  
  - Ability to delete or lock calendars.  
  - Change calendar color, visibility (public/private).
  - Export calendars as ICS files.
  - CalDAV sync (if enabled).
  - import ICS files.

- **Share Management:**  
  - View all share rules (calendar → user/link).  
  - Change permissions (read/write), delete shares.  

- **Change Admin Password:**  
  - Form with old and new password, password strength rules.  
  - Confirmation via re‑login.  

- **Logs & Statistics (optional):**  
  - Number of active users, public calendars, API requests.

---

## 5. User Web GUI

The user web GUI can be used by both `admin` and `user` roles; for regular users, the focus is on usage and personal calendars.

### 5.1 User Features

- **Calendar Overview:**  
  - List of user’s calendars (including shared calendars).  
  - Create, rename, delete own calendars.  
  - Change calendar color, visibility (public/private).
  - Export calendars as ICS files.
  - CalDAV sync (if enabled).
  - import ICS files.

- **Event Editing:**  
  - Web calendar view (day/week/month).  
  - Drag‑&‑drop events, GUI‑based creation of recurring events.  

- **Sharing Features:**  
  - View other users’ calendars if permitted.  
  - Share own calendars with other users (read/write).  
  - Generate anonymous links (token‑URL) with read‑only access.  

- **Change User Password:**  
  - Form similar to the admin one, but restricted to own account.

---

## 6. API Requirements

The API should serve both backend/frontend and external clients (e.g., mobile apps).

### 6.1 Authentication

- User login via API (username + password → JWT or session token).  
- Admin login differs by permissions (additional claims in JWT).

### 6.2 Resources and Endpoints

- **Users:**  
  - `GET /api/users` (admin‑only)  
  - `POST /api/users` (admin‑only)  
  - `PUT /api/users/:id` (admin‑only)  
  - `DELETE /api/users/:id` (admin‑only)  

- **Calendars:**  
  - `GET /api/calendars` (own + shared)  
  - `GET /api/calendars/:id`  
  - `POST /api/calendars` (new user calendar)  
  - `PUT /api/calendars/:id` (name, color, visibility)  
  - `DELETE /api/calendars/:id`  

- **Events:**  
  - `GET /api/calendars/:id/events` (with start/end, filters)  
  - `GET /api/events/:uid`  
  - `POST /api/calendars/:id/events`  
  - `PUT /api/events/:uid` (with ETag check)  
  - `DELETE /api/events/:uid` (with ETag check)  

- **Shares:**  
  - `GET /api/shares`  
  - `POST /api/shares` (create share: calendarId, userId/null, rights, token)  
  - `DELETE /api/shares/:id`  

- **QR Code Endpoint:**  
  - `GET /api/qr/calendar/:id` → PNG QR code for read‑only URL.  
  - `GET /api/qr/event/:uid` → QR code for single event.  

- **Public API (no login):**  
  - `GET /api/public/calendars` – list of public calendars.  
  - `GET /api/public/calendars/:id` – ICS data or JSON view.

---

## 7. Mobility & Integration

- **Android:**  
  - Use calendars via Android Calendar app combined with DAVx⁵ (CalDAV).  

- **iOS:**  
  - Set up CalDAV account in the iOS Calendar app.  

- **Web Browsers:**  
  - View and edit calendars via the web GUI.  

- **QR Code:**  
  - Event or calendar URL printable or displayable as QR code on a website.

---

## 8. Backend Architecture (Conceptual)

- **Web Framework:**  
  - Rust web framework such as **Axum** or **Tide** for HTTP API and web GUI hosting.  

- **Database:**  
  - SQLite or PostgreSQL with Rust bindings (e.g., `sqlx` or `diesel`).  

- **CalDAV Stack:**  
  - Integrate existing CalDAV crates (e.g., `dav_server` or `fast_dav_rs` components).  

- **Frontend Build:**  
  - Frontend (admin/user GUI) built via Webpack/Vite or similar, served as static files from the Rust server or hosted separately, communicating via API.

---

## 9. Release & Deployment Requirements

- **Container:**  
  - Docker image (or Podman) with the compiled binary.  

- **Configuration:**  
  - Configuration file (e.g., `config.yaml`) for:  
    - Database path/URL  
    - HTTPS certificate paths  
    - Initial admin user  

- **Update Mechanism:**  
  - Schema migrations (e.g., SQL migrations) for the database.  

- **Backup:**  
  - Documentation for backing up the database and optionally attachments.

---

## 10. Gitea Integration Requirements

- **Repository:**  
  - Single Gitea repository for backend source code.  

- **Branch Structure:**  
  - `main`/`master` for stable releases.  
  - `develop` for feature development.  

- **CI/CD:**  
  - Pipeline (e.g., Gitea Actions) for:  
    - Test (Rust tests)  
    - Build (Docker image)  
    - Documentation (e.g., API docs)

---

## 11. Optional Extensions (“Nice‑to‑have”)

- **Full‑text search in events** (title, description).  
- **Push notifications** for calendar changes (e.g., WebPush or DAV‑push‑like mechanism).  
- **LDAP integration** for user management.  
- **Backup export** of entire calendars in iCalendar format.
