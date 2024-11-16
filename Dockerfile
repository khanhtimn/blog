# Main Dockerfile
FROM leptos-builder-musl AS builder

# Build arguments for database configuration
ARG POSTGRES_USER
ARG POSTGRES_PASSWORD
ARG POSTGRES_HOST
ARG POSTGRES_PORT
ARG POSTGRES_DB

# Set environment variables for build time
ENV DATABASE_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}"

WORKDIR /work

# Make sure to have proper .dockerignore
COPY . .

# Create necessary directories
RUN mkdir -p target/site

# Run clippy checks
RUN cargo clippy -- -D warnings

# Setup Tailwind CSS
COPY tailwind.config.js .
COPY style/tailwind.css ./style/

# Build the application
RUN cargo leptos build --release

# Production stage
FROM scratch as app

# Runtime environment variables
ENV LEPTOS_OUTPUT_NAME=blog
ENV LEPTOS_SITE_ROOT=site
ENV LEPTOS_SITE_PKG_DIR=pkg
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_RELOAD_PORT=3001

# Database configuration at runtime
ENV POSTGRES_USER=""
ENV POSTGRES_PASSWORD=""
ENV POSTGRES_HOST=""
ENV POSTGRES_PORT=""
ENV POSTGRES_DB=""

USER 10001

WORKDIR /app

COPY --chown=10001:10001 --from=builder /work/target/site/ ./site/
COPY --chown=10001:10001 --from=builder /work/target/server/release/server .
COPY --chown=10001:10001 --from=builder /work/style/output.css ./site/

EXPOSE 3000

# Use an entrypoint script to build DATABASE_URL at runtime
COPY --chown=10001:10001 docker-entrypoint.sh /app/
RUN chmod +x /app/docker-entrypoint.sh

ENTRYPOINT ["/app/docker-entrypoint.sh"]
