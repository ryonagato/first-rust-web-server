# Use the official Rust image as the base image
FROM rust:1.79.0

# Install basic dependencies in one RUN command and clean up
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    git \
    nano \
    ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

    # Install cargo-watch for live reloading
RUN cargo install cargo-watch

# Set the Current Working Directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml ./

# Create a dummy main.rs file and build the dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

# Remove the dummy main.rs file
RUN rm -f src/main.rs

# Copy the source code to the container
COPY . .

# Command to run cargo-watch for live reloading
CMD ["cargo", "watch", "-x", "run"]
