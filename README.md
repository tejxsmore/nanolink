# NanoLink

**NanoLink** is a modern, minimal, and secure URL shortener built with **Rust** and **PostgreSQL** (using [Neon](https://neon.tech/) as the database platform). It provides both anonymous and authenticated URL shortening capabilities, allowing users to create either randomly-generated or custom short URLs.

## ✨ Features

- 🔐 **Authentication System**

  - Full user registration and login via username and password
  - Secure password hashing using `bcrypt`
  - JWT-based session management

- 🔗 **URL Shortening**

  - Anonymous users receive **random short URLs**
  - Authenticated users can **choose custom aliases**
  - URLs are stored efficiently in PostgreSQL with SQLx

- 📈 **URL Analytics**

  - View click counts per short URL
  - Track original long URLs
  - Timestamps for creation and clicks

- 👤 **User Dashboard**
  - Authenticated users can view all their created links
  - Easily manage or delete short URLs
  - Display click statistics and link metadata

## 🛠 Tech Stack

| Tech            | Description                         |
| --------------- | ----------------------------------- |
| Rust            | High-performance backend            |
| Axum            | Lightweight web framework           |
| SQLx            | Async, compile-time checked queries |
| Neon/PostgreSQL | Cloud-hosted PostgreSQL DB          |
| JWT             | Stateless authentication            |
| Bcrypt          | Password hashing                    |
| Tokio           | Async runtime                       |

## 📦 Dependencies

Here are some of the key crates used:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls", "macros", "uuid", "chrono"] }
dotenvy = "0.15"
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1", features = ["derive"] }
rand = "0.8"
anyhow = "1.0.98"
serde_json = "1.0"
axum = "0.8.4"
bcrypt = "0.15"
jsonwebtoken = "9.2"
```

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- A PostgreSQL database (Neon or local)
- [Docker](https://www.docker.com/) (optional, for local DB testing)

### Environment Variables

Create a `.env` file in the root:

```env
DATABASE_URL=postgres://<user>:<password>@<host>/<db>
JWT_SECRET=<your_jwt_secret>
```

### Running the Project

```bash
# Install dependencies
cargo build

# Run migrations (if using SQLx)
sqlx migrate run

# Start the server
cargo run
```

## 🔐 Authentication Flow

- **Signup**: Users register with a username and password. Passwords are hashed with `bcrypt`.
- **Login**: On successful login, a JWT token is returned.
- **Authorization**: Authenticated routes require a valid JWT token in the `Authorization` header.

## 📚 API Overview

| Method | Endpoint         | Description                            |
| ------ | ---------------- | -------------------------------------- |
| POST   | `/api/signup`    | Register a new user                    |
| POST   | `/api/login`     | Authenticate and receive JWT           |
| POST   | `/api/shorten`   | Create short URL (auth optional)       |
| GET    | `/u/:short_code` | Redirect to original URL               |
| GET    | `/api/profile`   | Get user's URL history (auth required) |
| DELETE | `/api/url/:id`   | Delete a URL (auth required)           |

## 📏 URL Logic

- Anonymous users receive short codes like: `abc123`
- Authenticated users can create codes like: `my-custom-link`
- Collision handling and validation included

## 📄 Roadmap

- [ ] Rate limiting for abuse prevention
- [ ] Expiry dates for links
- [ ] QR code generation
- [ ] Web frontend with Axum + Yew or Tauri

## 🧪 Testing

Run unit and integration tests with:

```bash
cargo test
```

## 🤝 Contributing

Contributions, feature requests, and feedback are welcome! Please open issues or PRs.

## 📜 License

This project is licensed under the MIT License.

> Built with ❤️ in Rust for speed, safety, and simplicity.
