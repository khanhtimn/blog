# Main Dockerfile
FROM ghcr.io/khanhtimn/cargo-leptos-builder-musl:latest AS builder

# Build arguments for database configuration
ARG POSTGRES_USER
ARG POSTGRES_PASSWORD
ARG POSTGRES_HOST
ARG POSTGRES_PORT
ARG POSTGRES_DB

# Set environment variables for build time
ENV DATABASE_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}"

WORKDIR /work

COPY . .

# Create necessary directories
RUN mkdir -p target/site

# Run clippy checks
# Now this is just annoying with current nightly rust
#RUN cargo clippy -- -D warnings

# Setup Tailwind CSS
COPY tailwind.config.js .
COPY style/tailwind.css ./style/

# Build the application
RUN npm i -D
RUN cargo leptos build --release -vv

# Production stage
FROM scratch as app

USER 10001

WORKDIR /app

COPY --chown=10001:10001 --from=builder /work/target/site/ ./site/
COPY --chown=10001:10001 --from=builder /work/target/release/server .
COPY --chown=10001:10001 --from=builder /work/style/tailwind.css ./site/
COPY --chown=10001:10001 --from=builder /work/Cargo.toml .
EXPOSE 3000

ENTRYPOINT ["/app/server"]
