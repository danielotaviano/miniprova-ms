# Use the official Rust image as a builder stage
FROM rust:1.79.0 as builder

# Create a new empty shell project
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files and download the dependencies
COPY ./Cargo.toml .
RUN echo "// dummy file" > dummy.rs
RUN cargo build --lib --release

COPY /src/ ./src
RUN cargo build --release

RUN cp ./target/release/exam /usr/local/bin/app

EXPOSE 3000

CMD ["app"]