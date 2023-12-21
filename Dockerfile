FROM debian:bookworm-slim
WORKDIR /app
ADD target/release/mrCache /app/mrCache
EXPOSE 50051
CMD ["/mrCache"]