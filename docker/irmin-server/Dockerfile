# Dockerfile for Irmin Bridge Server
FROM ocaml/opam:ubuntu-22.04-ocaml-4.14

# Switch to root to install system dependencies
USER root

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libffi-dev \
    libgmp-dev \
    libssl-dev \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Switch back to opam user
USER opam

# Set working directory
WORKDIR /home/opam/irmin-bridge

# Copy OCaml project files
COPY --chown=opam:opam dune-project ./
COPY --chown=opam:opam irmin-bridge.opam ./
COPY --chown=opam:opam ocaml-bridge ./ocaml-bridge/

# Install OCaml dependencies
RUN eval $(opam env) && \
    opam install -y --deps-only .

# Build the project
RUN eval $(opam env) && \
    dune build

# Install the executables
RUN eval $(opam env) && \
    dune install --prefix=/home/opam/.local

# Create data directory for Irmin store
RUN mkdir -p /home/opam/data/irmin_store

# Expose port
EXPOSE 8080

# Set environment
ENV PATH="/home/opam/.local/bin:$PATH"

# Create startup script
RUN echo '#!/bin/bash\n\
eval $(opam env)\n\
echo "Starting Irmin Bridge Server..."\n\
echo "Store path: $IRMIN_STORE_PATH"\n\
echo "Port: $IRMIN_PORT"\n\
exec irmin-bridge-server --port ${IRMIN_PORT:-8080} --store ${IRMIN_STORE_PATH:-/home/opam/data/irmin_store}\n\
' > /home/opam/start-server.sh && \
chmod +x /home/opam/start-server.sh

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:${IRMIN_PORT:-8080}/health || exit 1

# Default command
CMD ["/home/opam/start-server.sh"]