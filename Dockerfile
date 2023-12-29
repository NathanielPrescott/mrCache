# I am just learning Docker config and may not have the most effecient setup here.
# Start from a Rust base image
FROM rust:1.74 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin mrCache
WORKDIR /mrCache

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source tree
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/mrCache*
RUN cargo build --release

# Our second stage, that will be the final image
FROM ubuntu:latest

# We need to add the target architecture of Rust binaries
# to the Rust standard library search path in case of dynamically linked binaries
ENV LD_LIBRARY_PATH=/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib

# Install OpenSSL, needed for our application
RUN apt-get update \
    && apt-get install -y openssl \
    && apt-get clean

# Copy the build artifact from the builder stage and set the startup command
COPY --from=builder /mrCache/target/release/mrCache .

# Set the startup command to run your binary
CMD ["./mrCache"]