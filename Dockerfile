# Start with a slim Debian-based image for Rust
FROM rust:1.80-slim-bookworm as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/src/app

# Copy the Rust project files
COPY backend/Cargo.toml backend/Cargo.lock ./backend/
COPY backend/src ./backend/src
COPY backend/migrations ./backend/migrations

# Cache dependencies
WORKDIR /usr/src/app/backend
RUN cargo fetch

# Build the Rust backend
RUN cargo build --release

# Build the SolidJS frontend
WORKDIR /usr/src/app
COPY frontend ./frontend
WORKDIR /usr/src/app/frontend
RUN npm install
RUN npm run build

# Start a new stage for a smaller final image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-privileged user with no home directory and disabled password
RUN useradd -r -s /bin/false appuser

# Set the working directory
WORKDIR /usr/src/app

# Copy the built backend binary from the builder stage
COPY --from=builder /usr/src/app/backend/target/release/royce ./

# Copy the built frontend files
COPY --from=builder /usr/src/app/frontend/dist/public ./frontend/dist

# Set ownership and permissions
RUN chown -R appuser:appuser /usr/src/app && chmod -R 755 /usr/src/app

# Switch to non-privileged user
USER appuser

# Expose the port the backend runs on
EXPOSE 8000

# Command to run the application
CMD ["./royce"]