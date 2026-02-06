# --- Tauri Build Stage (Frontend + Backend) ---
FROM rust:1.85-slim AS tauri-builder
ARG USE_MIRROR=auto

# Conditionally use Aliyun mirror for APT
RUN if [ "$USE_MIRROR" = "true" ] || ( [ "$USE_MIRROR" = "auto" ] && ! timeout 3 bash -c "</dev/tcp/www.google.com/80" 2>/dev/null ); then \
    echo "Using Aliyun mirror for APT..."; \
    sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list.d/debian.sources || \
    sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list; \
    else \
    echo "Using default APT sources..."; \
    fi

# Install Node.js and build dependencies for Tauri
RUN apt-get update && apt-get install -y \
    curl \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 20
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    rm -rf /var/lib/apt/lists/*

# Use npm mirror if needed
RUN if [ "$USE_MIRROR" = "true" ] || ( [ "$USE_MIRROR" = "auto" ] && ! timeout 3 bash -c "</dev/tcp/www.google.com/80" 2>/dev/null ); then \
    echo "Using npm mirror..."; \
    npm config set registry https://registry.npmmirror.com; \
    else \
    echo "Using default npm registry..."; \
    fi

# Use Aliyun mirror for Cargo if needed (Sparse Index)
ENV CARGO_HTTP_MULTIPLEXING=false
RUN if [ "$USE_MIRROR" = "true" ] || ( [ "$USE_MIRROR" = "auto" ] && ! timeout 3 bash -c "</dev/tcp/www.google.com/80" 2>/dev/null ); then \
    echo "Using Aliyun mirror for Cargo..."; \
    mkdir -p /root/.cargo && \
    echo "[source.crates-io]\nreplace-with = 'aliyun'\n\n[source.aliyun]\nregistry = \"sparse+https://mirrors.aliyun.com/crates.io-index/\"" > /root/.cargo/config.toml; \
    else \
    echo "Using default Cargo registry..."; \
    fi

WORKDIR /app

# Copy package files and install dependencies
COPY package*.json ./
RUN npm install

# Invalidate cache when source changes (force rebuild)
# IMPORTANT: Must be BEFORE COPY to invalidate cache
ARG CACHEBUST=1
RUN echo "Cache bust: $CACHEBUST"

# Copy the entire project
COPY . .

# Build with Tauri (Frontend + Backend)
RUN npm run tauri build

# --- Final Runtime Stage ---
FROM debian:bookworm-slim
ARG USE_MIRROR=auto
WORKDIR /app

# Conditionally use Aliyun mirror for APT
RUN if [ "$USE_MIRROR" = "true" ] || ( [ "$USE_MIRROR" = "auto" ] && ! timeout 3 bash -c "</dev/tcp/www.google.com/80" 2>/dev/null ); then \
    echo "Using Aliyun mirror for APT..."; \
    sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list.d/debian.sources || \
    sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list; \
    else \
    echo "Using default APT sources..."; \
    fi

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    libsqlite3-0 \
    ca-certificates \
    libgtk-3-0 \
    libwebkit2gtk-4.1-0 \
    libayatana-appindicator3-1 \
    librsvg2-2 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from Tauri builder
COPY --from=tauri-builder /app/src-tauri/target/release/antigravity_tools /app/antigravity-tools

# Copy frontend dist from Tauri builder
COPY --from=tauri-builder /app/dist /app/dist

# Set environment variables
ENV ABV_DIST_PATH=/app/dist
ENV RUST_LOG=info
ENV PORT=8046

# Expose the proxy/admin port
EXPOSE 8046

# Run the application in headless mode
ENTRYPOINT ["/app/antigravity-tools", "--headless"]
