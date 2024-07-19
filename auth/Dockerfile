# Use the official Rust image as a builder stage
FROM rust:1.79.0 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files and download the dependencies
COPY ./Cargo.toml ./
RUN cargo build --release

# Copy the source and build the application
COPY ./src ./src
RUN cargo build --release

# COPY ./target/release/auth /usr/local/bin/app
RUN cp ./target/release/auth /usr/local/bin/app
RUN rm -rf /app

# Expose port 3000
EXPOSE 3000

CMD ["app"]