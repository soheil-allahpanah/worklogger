# Worklogger

A terminal-based work log for tracking time spent on tasks. Log sessions with Jalali dates, durations, descriptions, and tags, then browse, filter, and export them from a keyboard-driven TUI or an HTTP API backed by PostgreSQL.

Built as a Rust workspace with a small hexagonal architecture: domain logic stays independent of the database and UI, so the same use cases power both the TUI and the HTTP API.

## Features

- **Interactive TUI** — browse, add, open, and soft-delete worklog entries from the terminal
- **Jalali calendar** — dates are stored in UTC but displayed in the Jalali calendar (Asia/Tehran)
- **Flexible durations** — enter `2h30m`, `45m`, or raw seconds
- **Tag support** — comma-separated tags with stable, hash-based colors in the table
- **Search DSL** — filter by tag, description, date range, duration, and ID from the bottom search bar
- **Excel export** — export filtered worklogs to a styled `.xlsx` file from the TUI (`e`) or the HTTP API
- **Soft deletes** — deleted entries are retained with a `deleted_at` timestamp
- **HTTP API** — create, filter, delete, and export worklogs over REST
- **Clean architecture** — `core` domain, `use_cases` application layer, `infrastructure` persistence, `tui` and `api` presentation

## Architecture

```
┌─────────────┐     ┌─────────────┐
│     tui     │     │     api     │
└──────┬──────┘     └──────┬──────┘
       │                   │
       └─────────┬─────────┘
                 ▼
         ┌───────────────┐
         │   use_cases   │  Create, filter, get, delete, export
         └───────┬───────┘
                 │
       ┌─────────┴─────────┐
       ▼                   ▼
┌─────────────┐     ┌──────────────┐
│    core     │     │infrastructure│
│  (domain)   │◄────│  PostgreSQL  │
└─────────────┘     └──────────────┘
       ▲
       │
┌─────────────┐
│   common    │  Filters, pagination
└─────────────┘
```


| Crate                               | Role                                                                   |
| ----------------------------------- | ---------------------------------------------------------------------- |
| `[core](core/)`                     | Domain entities (`Worklog`), value objects, repository traits          |
| `[common](common/)`                 | Shared filter types and pagination helpers                             |
| `[use_cases](use_cases/)`           | Application commands, validation, use case orchestration, Excel export |
| `[infrastructure](infrastructure/)` | PostgreSQL repository via `sqlx`                                       |
| `[tui](tui/)`                       | Ratatui terminal UI                                                    |
| `[api](api/)`                       | HTTP API (Axum) — create, filter, delete, export                       |


## Prerequisites

- **Rust** 1.85+ (edition 2024)
- **PostgreSQL** 14+
- **sqlx-cli** (optional, for migrations and compile-time query checking)

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

## Getting started

### 1. Database

Create a PostgreSQL database and user, then apply the schema migration:

```bash
createdb worklogger_service   # or any name you prefer

# Point sqlx at your database
export DATABASE_URL=postgres://USER:PASSWORD@127.0.0.1:5432/worklogger_service

sqlx migrate run --source infrastructure/migrations
```

The migration creates a `worklogs` table with UUID primary key, timestamps, tags (`TEXT[]`), duration, description, and soft-delete support.

### 2. Environment

Copy the example env file and set your connection string:

```bash
cp .env.example .env
```

```env
DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/worklog

# TUI (HTTP client — API must be running)
WORKLOGGER_BASE_URL=http://127.0.0.1:3000
WORKLOGGER_TOKEN=wl_your_device_token

# Optional
# HOST=127.0.0.1
# PORT=3000
# WORKLOGGER_EXPORT_DIR=~/Download
```

The API reads `DATABASE_URL` at startup. The TUI talks to the API via the SDK and requires `WORKLOGGER_BASE_URL` and `WORKLOGGER_TOKEN`. It also accepts `WORKLOGGER_EXPORT_DIR` to choose where Excel files are saved (default: `~/Download`).

### 3. Run the TUI

Start the API first (see below), then from the repository root:

```bash
export WORKLOGGER_BASE_URL=http://127.0.0.1:3000
export WORKLOGGER_TOKEN=wl_your_device_token
cargo run -p tui
```

Or build a release binary:

```bash
cargo build -p tui --release
./target/release/tui
```

### 4. Run the API

```bash
export DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/worklog
cargo run -p api
```

Or build a release binary:

```bash
cargo build -p api --release
./target/release/api
```

Optional environment variables:


| Variable                | Default      | Description                              |
| ----------------------- | ------------ | ---------------------------------------- |
| `HOST`                  | `127.0.0.1`  | API bind address                         |
| `PORT`                  | `3000`       | API listen port                          |
| `WORKLOGGER_BASE_URL`   | —            | TUI: API base URL (required)             |
| `WORKLOGGER_TOKEN`      | —            | TUI: device bearer token (required)      |
| `WORKLOGGER_EXPORT_DIR` | `~/Download` | TUI Excel export directory               |


## TUI guide

### Layout

```
        WORK LOGGER v0.1.0 [Main View | Logged: N entries]
┌──────────┬──────────┬─────────────────────┬──────────────────────────────────┐
│ Date     │ Duration │ Description         │ Tags                             │
└──────────┴──────────┴─────────────────────┴──────────────────────────────────┘
│ ...      │ ...      │ ...                 │ rust, meeting                    │
│ ...      │ ...      │ ...                 │ dev                              │

┌──────────────────────────────────────────────────────────────────────────────┐
│ / tag:rust desc:"meeting"                                                    │
└──────────────────────────────────────────────────────────────────────────────┘
  q  QUIT  |  /  SEARCH  |  n  ADD  |  d  DELETE  |  e  EXPORT  |  j/k  NAVIGATE
```

### Keybindings


| Key                    | Action                                 |
| ---------------------- | -------------------------------------- |
| `j` / `k` or `↓` / `↑` | Move selection                         |
| `/`                    | Focus search bar                       |
| `n` or `a`             | Add a new worklog                      |
| `o`                    | Open selected entry (detail view)      |
| `d`                    | Delete selected entry                  |
| `e`                    | Export current search results to Excel |
| `q` or `Ctrl+c`        | Quit                                   |


**Search mode:** `Enter` applies the filter, `Esc` cancels.

**Add dialog:** `Tab` / `Shift+Tab` move between fields, `Enter` saves, `Esc` cancels.

**Delete dialog:** `Enter` confirms, `Esc` cancels.

**Detail view:** `Esc` closes.

### Adding an entry


| Field       | Format                            | Notes                                           |
| ----------- | --------------------------------- | ----------------------------------------------- |
| Date        | `YYYY/MM/DD` or `YYYY-MM-DD`      | Jalali calendar; leave blank for today (Tehran) |
| Duration    | `2h30m`, `45m`, `90s`, or seconds | Must be > 0 and < 24 h                          |
| Description | free text                         | Required                                        |
| Tags        | comma-separated                   | At least one tag required                       |


### Search DSL

Press `/` to edit the search bar. Tokens are space-separated; use quotes for values with spaces (parsed via `shlex`).


| Token       | Example                       | Meaning                                |
| ----------- | ----------------------------- | -------------------------------------- |
| `tag:`      | `tag:rust,dev`                | Include entries with any of these tags |
| `-tag:`     | `-tag:meeting`                | Exclude these tags                     |
| `desc:`     | `desc:"fix bug"`              | Description contains text              |
| `date:`     | `date:1403/01/01..1403/01/31` | Jalali date range (inclusive)          |
| `date:`     | `date:>=1403/01/01`           | On or after date                       |
| `date:`     | `date:<=1403/01/31`           | On or before date                      |
| `duration:` | `duration:1h..4h`             | Duration range                         |
| `id:`       | `id:550e8400-e29b-...`        | Filter by UUID                         |


Example:

```
tag:rust desc:"code review" date:1403/06/01..
```

### Export to Excel

Press `e` in the main view to export the **current search results** to an Excel file. The export uses the same filter as the search bar (an empty search exports all worklogs, up to 100,000 rows).

Files are written to `WORKLOGGER_EXPORT_DIR` (default `~/Download`) with a timestamped name such as `worklogs_20240612_153045.xlsx`. A status message confirms the path and row count.

Each spreadsheet includes columns for ID, Jalali date, duration, description, and color-coded tags, with a title row, frozen header, and autofilter — the same layout produced by the HTTP API export endpoints.

## API guide

The HTTP API exposes the same use cases as the TUI: create, filter, soft-delete, and export worklogs to Excel.

Default base URL: `http://127.0.0.1:3000`

### Run the API

Set `DATABASE_URL`, then start the server:

```bash
export DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/worklog
cargo run -p api
```

Release binary:

```bash
cargo build -p api --release
./target/release/api
```

Optional environment variables:


| Variable | Default     | Description  |
| -------- | ----------- | ------------ |
| `HOST`   | `127.0.0.1` | Bind address |
| `PORT`   | `3000`      | Listen port  |


### Docker

Build the image from the repository root:

```bash
docker build -f api/Dockerfile -t worklogger-api .
```

Run against an external PostgreSQL service (migrations are not applied by the container):

```bash
docker run --rm -p 3000:3000 \
  -e DATABASE_URL='postgres://USER:PASSWORD@db-host:5432/worklog' \
  worklogger-api
```

If PostgreSQL runs in another Docker container (for example one named `pg`), attach both to a shared network and use the container name as the host:

```bash
docker network create worklogger-net
docker network connect worklogger-net pg

docker run --rm -p 3000:3000 --network worklogger-net \
  -e DATABASE_URL='postgres://USER:PASSWORD@pg:5432/worklog' \
  worklogger-api
```

Inside a container, `localhost` refers to that container itself — use the database container name or `host.docker.internal` (with `--add-host=host.docker.internal:host-gateway` on Linux) when Postgres runs on the host.

### Endpoints


| Method   | Path               | Description                         |
| -------- | ------------------ | ----------------------------------- |
| `GET`    | `/health`          | Health check                        |
| `POST`   | `/worklogs`        | Create a worklog                    |
| `DELETE` | `/worklogs/{id}`   | Soft-delete a worklog               |
| `GET`    | `/worklogs`        | Filter via query parameters         |
| `POST`   | `/worklogs/filter` | Filter via JSON body                |
| `GET`    | `/worklogs/export` | Export to XLSX via query parameters |
| `POST`   | `/worklogs/export` | Export to XLSX via JSON body        |


Errors return JSON: `{ "error": "...", "details": [...] }` with `400` for validation failures, `404` when a worklog is not found, and `500` for internal errors.

### Create a worklog

```bash
curl -X POST http://127.0.0.1:3000/worklogs \
  -H 'Content-Type: application/json' \
  -d '{
    "jalali_date": "1403/06/01",
    "duration_secs": 3600,
    "tags": ["rust", "api"],
    "description": "Implemented HTTP API"
  }'
```


| Field           | Type             | Notes                                                    |
| --------------- | ---------------- | -------------------------------------------------------- |
| `jalali_date`   | string, optional | `YYYY/MM/DD` or `YYYY-MM-DD`; defaults to today (Tehran) |
| `duration_secs` | number           | Must be > 0 and < 86_400                                 |
| `tags`          | string array     | At least one tag required                                |
| `description`   | string           | Required                                                 |


Response (`201 Created`):

```json
{ "id": "550e8400-e29b-41d4-a716-446655440000" }
```

### Filter worklogs

**Query parameters** (`GET /worklogs`):


| Parameter       | Example       | Meaning                     |
| --------------- | ------------- | --------------------------- |
| `tags`          | `rust,dev`    | Include any of these tags   |
| `exclude_tags`  | `meeting`     | Exclude these tags          |
| `ids`           | `uuid1,uuid2` | Include these IDs           |
| `exclude_ids`   | `uuid3`       | Exclude these IDs           |
| `description`   | `fix bug`     | Description contains text   |
| `date_from`     | `1403/01/01`  | Jalali date on or after     |
| `date_to`       | `1403/01/31`  | Jalali date on or before    |
| `duration_from` | `1h`          | Minimum duration (`xhymzs`) |
| `duration_to`   | `4h`          | Maximum duration            |
| `page`          | `1`           | Page number (default `1`)   |
| `size`          | `20`          | Page size (default `20`)    |


```bash
curl 'http://127.0.0.1:3000/worklogs?tags=rust&page=1&size=20'
```

**JSON body** (`POST /worklogs/filter`):

```bash
curl -X POST http://127.0.0.1:3000/worklogs/filter \
  -H 'Content-Type: application/json' \
  -d '{
    "tags": { "in_list": ["rust"] },
    "description": { "contains": "review" },
    "date": { "from": "1403/06/01", "to": "1403/06/30" },
    "paging": { "page": 1, "size": 20 }
  }'
```

Paginated responses include `items`, `total_items`, `total_pages`, `current_page`, and `page_size`. Each item includes UTC timestamps, a Jalali date, duration in seconds and as `2h30m`, tags, and description.

### Delete a worklog

```bash
curl -X DELETE http://127.0.0.1:3000/worklogs/550e8400-e29b-41d4-a716-446655440000
```

Returns `204 No Content` on success.

### Export to Excel

Uses the same filters as the filter endpoints. Returns a styled `.xlsx` file with columns **ID**, **Date** (Jalali), **Duration**, **Description**, and **Tags**. Up to 100,000 rows per export.

```bash
curl -OJ 'http://127.0.0.1:3000/worklogs/export?tags=rust'
```

Or with a JSON filter:

```bash
curl -X POST http://127.0.0.1:3000/worklogs/export \
  -H 'Content-Type: application/json' \
  -d '{"tags": { "in_list": ["rust"] }}' \
  -o worklogs.xlsx
```

The response includes `Content-Disposition: attachment`, `Content-Type: application/vnd.openxmlformats-officedocument.spreadsheetml.sheet`, and an `X-Row-Count` header with the number of exported rows.

## Development

### Build the workspace

```bash
cargo build
```

### Run tests

```bash
cargo test
```

### sqlx offline builds

Query macros are checked against a live database by default. To compile without PostgreSQL running, use the checked-in query cache:

```bash
SQLX_OFFLINE=true cargo build
```

To refresh the cache after changing SQL:

```bash
export DATABASE_URL=postgres://...
cargo sqlx prepare --workspace
```

## License

No license file is included yet. Add one before distributing.