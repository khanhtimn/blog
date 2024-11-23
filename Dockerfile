# Main Dockerfile
FROM ghcr.io/khanhtimn/cargo-leptos-builder-musl:latest AS builder

WORKDIR /work
COPY . .

RUN mkdir -p target/site

# Run clippy checks
# Now this is just annoying with current nightly rust
# RUN cargo clippy -- -D warnings

COPY tailwind.config.js .
COPY style/tailwind.css ./style/

RUN npm i -D
RUN cargo leptos build --release -vv

#FROM scratch as app
FROM alpine:3.18.2 as app


USER 10001

WORKDIR /app

COPY --chown=10001:10001 --from=builder /work/target/site/ ./site/
COPY --chown=10001:10001 --from=builder /work/target/release/server .
COPY --chown=10001:10001 --from=builder /work/style/tailwind.css ./site/
COPY --chown=10001:10001 --from=builder /work/Cargo.toml .
EXPOSE 3000

ENTRYPOINT ["/app/server"]
