# Dockerfile for IrminView Application
FROM rust:1.75-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libgtk-3-dev \
    libglib2.0-dev \
    libwebkit2gtk-4.1-dev \
    libjavascriptcoregtk-4.1-dev \
    libsoup-3.0-dev \
    build-essential \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Rust project files
COPY src-tauri/Cargo.toml src-tauri/Cargo.lock ./src-tauri/
COPY src-tauri/src ./src-tauri/src/
COPY src-tauri/tauri.conf.json ./src-tauri/
COPY src-tauri/build.rs ./src-tauri/
COPY src-tauri/icons ./src-tauri/icons/

# Copy frontend files
COPY frontend ./frontend/

# Set environment variables for Docker deployment
ENV IRMIN_USE_HTTP=true
ENV IRMIN_SERVER_URL=http://irmin-server:8080
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig

# Build the application
WORKDIR /app/src-tauri
RUN cargo build --release

# Create startup script
RUN echo '#!/bin/bash\n\
echo "Starting IrminView Application..."\n\
echo "Irmin Server URL: $IRMIN_SERVER_URL"\n\
echo "Using HTTP mode: $IRMIN_USE_HTTP"\n\
exec ./target/release/irmin-view\n\
' > /app/start-app.sh && \
chmod +x /app/start-app.sh

# Expose port (if needed for web interface)
EXPOSE 3000

# Default command
CMD ["/app/start-app.sh"]