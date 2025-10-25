# EMP Backend - Rust API Server

A robust Rust backend API server built with Actix Web, featuring authentication, database management, Redis caching, and MQTT messaging.

## 🚀 Features

- **Authentication & Authorization**: JWT-based authentication with refresh tokens
- **Database**: PostgreSQL with SeaORM and connection pooling
- **Caching**: Redis integration for user data and token blacklisting
- **Messaging**: MQTT support for real-time communication
- **Validation**: Comprehensive input validation using validator crate
- **Error Handling**: Custom error types with proper HTTP status codes
- **Logging**: Structured logging with file and console output
- **Health Checks**: System health monitoring endpoints
- **API Documentation**: OpenAPI/Swagger documentation

## 📋 Prerequisites

- Rust 1.70+ 
- PostgreSQL 13+
- Redis 6+
- MQTT Broker (optional)

## 🛠️ Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd rust-emp-be
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Set up environment variables**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

4. **Set up the database**
   ```bash
   # Create PostgreSQL database
   createdb emp_be
   
   # Run migrations (if you have them)
   # cargo run --bin migrate
   ```

## ⚙️ Configuration

Create a `.env` file with the following variables:

```env
# Database Configuration
DATABASE_URL=postgres://username:password@localhost:5432/emp_be

# Redis Configuration  
REDIS_URL=redis://localhost:6379

# MQTT Configuration
MQTT_BROKER=localhost
MQTT_PORT=1883
MQTT_CLIENT_ID=server_receiver

# Security Configuration (IMPORTANT: Change these!)
JWT_SECRET=your_super_secret_jwt_key_here_make_it_long_and_random
JWT_EXPIRY_MINUTES=15
REFRESH_TOKEN_EXPIRY_DAYS=7

# Argon2 Configuration
ARGON2_MEMORY_COST=4096
ARGON2_TIME_COST=3
ARGON2_PARALLELISM=1

# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# Cookie Configuration
AUTH_COOKIE=auth_token
REFRESH_COOKIE=refresh_token
```

## 🏃‍♂️ Running the Application

### Development
```bash
cargo run
```

### Production
```bash
cargo build --release
./target/release/emp-be
```

The server will start at `http://localhost:8080`

## 📚 API Documentation

Once the server is running, you can access the API documentation at:
- **Swagger UI**: `http://localhost:8080/docs/`
- **OpenAPI JSON**: `http://localhost:8080/api-docs/openapi.json`

## 🔗 API Endpoints

### Authentication
- `POST /auth/register` - Register a new user
- `POST /auth/login` - User login
- `POST /auth/refresh` - Refresh access token
- `POST /auth/logout` - User logout
- `GET /auth/profile` - Get user profile (protected)
- `PUT /auth/profile` - Update user profile (protected)
- `POST /auth/change-password` - Change password (protected)
- `POST /auth/forgot-password` - Request password reset
- `POST /auth/reset-password` - Reset password with token
- `GET /auth/users` - Get all users (admin)

### System
- `GET /health` - Health check endpoint

## 🏗️ Project Structure

```
src/
├── app.rs                 # Application setup and server configuration
├── main.rs               # Entry point
├── lib.rs                # Library exports
├── config/               # Configuration modules
│   ├── load_env.rs       # Environment variable loading
│   ├── security.rs       # Security configuration
│   └── logging.rs        # Logging configuration
├── errors/               # Custom error types
│   └── mod.rs           # Error definitions and handling
├── infra/               # Infrastructure layer
│   ├── cache/           # Redis caching
│   ├── database/        # Database connections
│   └── messaging/       # MQTT messaging
└── modules/             # Business logic modules
    ├── middlewares/     # HTTP middlewares
    ├── shared/          # Shared utilities
    ├── system/          # System-related functionality
    └── user/            # User management
        ├── handler/     # HTTP handlers
        ├── model/       # Database models
        ├── repository/  # Data access layer
        ├── schemas/     # Request/response schemas
        ├── service/     # Business logic
        └── utils/       # Utility functions
```

## 🔒 Security Features

- **JWT Authentication**: Secure token-based authentication
- **Password Hashing**: Argon2 password hashing
- **Input Validation**: Comprehensive validation for all inputs
- **Token Blacklisting**: Redis-based token invalidation
- **Secure Cookies**: HttpOnly, Secure cookies for token storage
- **Environment-based Secrets**: No hardcoded secrets

## 📊 Monitoring & Logging

- **Structured Logging**: JSON-formatted logs with timestamps
- **File Logging**: Daily rotating log files in `logs/` directory
- **Health Checks**: System health monitoring
- **Error Tracking**: Comprehensive error logging and handling

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## 🚀 Deployment

### Docker (Recommended)
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/emp-be /usr/local/bin/emp-be
EXPOSE 8080
CMD ["emp-be"]
```

### Manual Deployment
1. Build the release version: `cargo build --release`
2. Copy the binary to your server
3. Set up environment variables
4. Run the application

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Commit your changes: `git commit -am 'Add feature'`
4. Push to the branch: `git push origin feature-name`
5. Submit a pull request

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🆘 Troubleshooting

### Common Issues

1. **Database Connection Failed**
   - Check PostgreSQL is running
   - Verify DATABASE_URL in .env
   - Ensure database exists

2. **Redis Connection Failed**
   - Check Redis is running
   - Verify REDIS_URL in .env

3. **JWT Secret Not Set**
   - Ensure JWT_SECRET is set in .env
   - Use a strong, random secret key

4. **Port Already in Use**
   - Change SERVER_PORT in .env
   - Or kill the process using the port

### Logs
Check the application logs in the `logs/` directory for detailed error information.

## 📞 Support

For support and questions, please open an issue in the repository.
