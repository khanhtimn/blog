#!/bin/sh

# Construct DATABASE_URL from environment variables
export DATABASE_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}"

# Start the server
exec /app/blog
