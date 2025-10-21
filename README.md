# Awesome Rust Shuttle

A production-ready Rust API template using [Shuttle](https://www.shuttle.rs/) for seamless deployment. This template provides a solid foundation for building REST APIs with best practices baked in.

## 🚀 Features

- **[Actix-web](https://actix.rs/)** - Fast and powerful web framework
- **PostgreSQL** - Robust database with SQLx for type-safe queries
- **[Shuttle](https://www.shuttle.rs/)** - Deploy for free with minimal configuration
- **Devcontainer** - Full Docker Compose development environment
- **Pre-configured Middleware**
  - CORS with customizable settings
  - Request logging
  - Rate limiting support (actix-governor)
- **Error Handling** - Structured error types and responses
- **Secrets Management** - Built-in secrets handling via Shuttle
- **Database Migrations** - SQLx migrations support
- **Health Check** - Ready-to-use health check endpoint
- **Common Dependencies**
  - JWT authentication (jsonwebtoken)
  - Request validation (validator)
  - HTTP client (reqwest)
  - OpenAPI documentation (utoipa)

## 📋 Prerequisites

Choose one of the following setups:

### Option 1: Devcontainer (Recommended)

- Docker and Docker Compose
- VS Code with Dev Containers extension

### Option 2: Local Development

- Rust (latest stable)
- PostgreSQL
- [Shuttle CLI](https://docs.shuttle.rs/introduction/installation) (for deployment)

## 🏁 Quick Start

### Using Devcontainer

1. Clone this repository:

```bash
git clone https://github.com/cadunass/awesome-rust-shuttle.git
cd awesome-rust-shuttle
```

2. Open in VS Code and reopen in container when prompted, or use the command palette:

   - `Ctrl+Shift+P` → "Dev Containers: Reopen in Container"

3. The devcontainer includes PostgreSQL and all required tools. Everything will be set up automatically.

4. Copy the example secrets file:

```bash
cp example.Secrets.toml Secrets.toml
```

5. Run the application:

```bash
cargo shuttle run
```

6. Test the health check:

```bash
curl http://localhost:8000/health-check
```

### Local Development (without Devcontainer)

1. Clone and enter the repository:

```bash
git clone https://github.com/cadunass/awesome-rust-shuttle.git
cd awesome-rust-shuttle
```

2. Set up PostgreSQL and create a database

3. Copy and configure secrets:

```bash
cp example.Secrets.toml Secrets.toml
```

4. Update `Secrets.toml` with your database URL:

```toml
DATABASE_URL = 'postgres://username:password@localhost:5432/your_database'
```

5. Install Shuttle CLI:

```bash
cargo install cargo-shuttle
```

6. Run the application:

```bash
cargo shuttle run
```

## ⚙️ Configuration

### Secrets Management

The application uses `Secrets.toml` for local development. This file is gitignored for security.

1. Copy the example file:

```bash
cp example.Secrets.toml Secrets.toml
```

2. Edit `Secrets.toml` with your configuration:

```toml
DATABASE_URL = 'postgres://user:password@host:port/database'
# Add other secrets as needed
```

### Database Configuration

The project uses SQLx for database operations. Migrations are automatically run on startup (see `src/main.rs:25-28`).

To create new migrations:

```bash
sqlx migrate add <migration_name>
```

## 🌐 Deployment

### Deploy to Shuttle

1. Install the Shuttle CLI if you haven't already:

```bash
cargo install cargo-shuttle
```

2. Login to Shuttle:

```bash
cargo shuttle login
```

3. Initialize your project (if not already done):

```bash
cargo shuttle project start
```

4. Set production secrets:

```bash
cargo shuttle secrets set DATABASE_URL='your_production_database_url'
```

5. Deploy:

```bash
cargo shuttle deploy
```

**Note:** Shuttle offers a generous free tier, making it perfect for hobby projects and prototypes. The database and hosting are provisioned automatically.

## 📁 Project Structure

```
awesome-rust-shuttle/
├── src/
│   ├── main.rs           # Application entry point and Shuttle configuration
│   ├── lib.rs            # Library exports
│   ├── config.rs         # Configuration structures
│   ├── errors.rs         # Error types and handling
│   ├── utils.rs          # Shared utilities (AppState, etc.)
│   ├── routes/           # API route handlers
│   │   ├── mod.rs
│   │   └── health_check.rs
│   └── modules/          # Business logic modules
├── migrations/           # Database migrations
├── .devcontainer/        # Devcontainer configuration
├── Cargo.toml           # Dependencies and project metadata
├── example.Secrets.toml # Example secrets file
└── README.md
```
