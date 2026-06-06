# Worklogger

A terminal-based work log for tracking time spent on tasks. Log sessions with Jalali dates, durations, descriptions, and tags, then browse and filter them from a keyboard-driven TUI backed by PostgreSQL.

Built as a Rust workspace with a small hexagonal architecture: domain logic stays independent of the database and UI, so the same use cases can power the TUI today and an HTTP API later.

## Features

- **Interactive TUI** — browse, add, open, and soft-delete worklog entries from the terminal
- **Jalali calendar** — dates are stored in UTC but displayed in the Jalali calendar (Asia/Tehran)
- **Flexible durations** — enter `2h30m`, `45m`, or raw seconds
- **Tag support** — comma-separated tags with stable, hash-based colors in the table
- **Search DSL** — filter by tag, description, date range, duration, and ID from the bottom search bar
- **Soft deletes** — deleted entries are retained with a `deleted_at` timestamp
- **Clean architecture** — `core` domain, `use_cases` application layer, `infrastructure` persistence, `tui` presentation

## Architecture

```
┌─────────────┐     ┌─────────────┐
│     tui     │     │     api     │  (placeholder)
└──────┬──────┘     └──────┬──────┘
       │                   │
       └─────────┬─────────┘
                 ▼
         ┌───────────────┐
         │   use_cases   │  Create, filter, get, delete
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

| Crate | Role |
|-------|------|
| [`core`](core/) | Domain entities (`Worklog`), value objects, repository traits |
| [`common`](common/) | Shared filter types and pagination helpers |
| [`use_cases`](use_cases/) | Application commands, validation, and use case orchestration |
| [`infrastructure`](infrastructure/) | PostgreSQL repository via `sqlx` |
| [`tui`](tui/) | Ratatui terminal UI — the main runnable binary today |
| [`api`](api/) | HTTP API scaffold (not implemented yet) |

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
```

The TUI reads `DATABASE_URL` at startup.

### 3. Run the TUI

From the repository root:

```bash
cargo run -p tui
```

Or build a release binary:

```bash
cargo build -p tui --release
./target/release/tui
```

## TUI guide

### Layout

```
        WORK LOGGER v0.1.0 [Main View | Logged: N entries]
┌──────────┬──────────┬─────────────────────┬──────────────────┐
│ Date     │ Duration │ Description         │ Tags             │
└──────────┴──────────┴─────────────────────┴──────────────────┘
│ ...      │ ...      │ ...                 │ rust, meeting    │
│ ...      │ ...      │ ...                 │ dev              │

┌───────────────────────────────────────────────────────────────┐
│ / tag:rust desc:"meeting"                                     │
└───────────────────────────────────────────────────────────────┘
  q  QUIT  |  /  SEARCH  |  n  ADD  |  d  DELETE  |  j/k  NAVIGATE
```

### Keybindings

| Key | Action |
|-----|--------|
| `j` / `k` or `↓` / `↑` | Move selection |
| `/` | Focus search bar |
| `n` or `a` | Add a new worklog |
| `o` | Open selected entry (detail view) |
| `d` | Delete selected entry |
| `q` or `Ctrl+c` | Quit |

**Search mode:** `Enter` applies the filter, `Esc` cancels.

**Add dialog:** `Tab` / `Shift+Tab` move between fields, `Enter` saves, `Esc` cancels.

**Delete dialog:** `Enter` confirms, `Esc` cancels.

**Detail view:** `Esc` closes.

### Adding an entry

| Field | Format | Notes |
|-------|--------|-------|
| Date | `YYYY/MM/DD` or `YYYY-MM-DD` | Jalali calendar; leave blank for today (Tehran) |
| Duration | `2h30m`, `45m`, `90s`, or seconds | Must be > 0 and < 24 h |
| Description | free text | Required |
| Tags | comma-separated | At least one tag required |

### Search DSL

Press `/` to edit the search bar. Tokens are space-separated; use quotes for values with spaces (parsed via `shlex`).

| Token | Example | Meaning |
|-------|---------|---------|
| `tag:` | `tag:rust,dev` | Include entries with any of these tags |
| `-tag:` | `-tag:meeting` | Exclude these tags |
| `desc:` | `desc:"fix bug"` | Description contains text |
| `date:` | `date:1403/01/01..1403/01/31` | Jalali date range (inclusive) |
| `date:` | `date:>=1403/01/01` | On or after date |
| `date:` | `date:<=1403/01/31` | On or before date |
| `duration:` | `duration:1h..4h` | Duration range |
| `id:` | `id:550e8400-e29b-...` | Filter by UUID |

Example:

```
tag:rust desc:"code review" date:1403/06/01..
```

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

### Project layout (TUI)

The TUI follows an Elm-style loop: input → message → update → view.

```
tui/src/
├── main.rs           # Entry point, wiring
├── app.rs            # Shared model and runtime
├── ui.rs             # Root layout composition
├── components/       # Table, search bar, help bar
├── dialogs/          # Add, delete, detail modals
├── search_dsl.rs     # Search bar → FilterWorklogsCommand
├── format.rs         # Jalali dates and duration display
└── theme.rs          # Colors and layout helpers
```

## Data model

Each worklog stores:

| Column | Type | Description |
|--------|------|-------------|
| `id` | UUID | Primary key |
| `datetime` | `TIMESTAMPTZ` | When the work happened |
| `duration` | `INTERVAL` | Length of the session |
| `tags` | `TEXT[]` | Zero or more labels |
| `description` | `TEXT` | What you worked on |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | Audit timestamps |
| `deleted_at` | `TIMESTAMPTZ` | Set on soft delete |

## Roadmap

- [ ] HTTP API (`api` crate)
- [ ] Automated migration on startup
- [ ] Export / reporting

## License

No license file is included yet. Add one before distributing.
