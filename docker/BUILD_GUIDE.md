# 🏗️ Antigravity Manager Docker Build Guide

This guide explains how to build the Antigravity Manager Docker image using the unified Tauri build process.

## Build Architecture

The Dockerfile uses a multi-stage build approach with `npm run tauri build`:

```
┌─────────────────────────────────────────────┐
│  Stage 1: Tauri Builder                     │
│  - Base: rust:1.85-slim                     │
│  - Install Node.js 20 + Build dependencies  │
│  - Run: npm run tauri build                 │
│    ├─> Vite builds frontend (React + TS)   │
│    └─> Cargo builds backend (Rust)         │
│  - Output: dist/ + target/release/binary    │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│  Stage 2: Runtime                           │
│  - Base: debian:bookworm-slim               │
│  - Copy binary + frontend assets            │
│  - Install runtime libraries only           │
│  - Expose port 8046                         │
└─────────────────────────────────────────────┘
```

## Quick Build Commands

### Standard Build (Auto-detect mirrors)
```bash
docker build -t antigravity-manager:latest -f docker/Dockerfile .
```

### Build with China Mirrors (Faster in China)
```bash
docker build --build-arg USE_MIRROR=true -t antigravity-manager:latest -f docker/Dockerfile .
```

### Build without Mirrors (International)
```bash
docker build --build-arg USE_MIRROR=false -t antigravity-manager:latest -f docker/Dockerfile .
```

### Build with Custom Tag
```bash
docker build -t antigravity-manager:v4.2.0 -f docker/Dockerfile .
```

## Build Arguments

| Argument | Default | Description |
|----------|---------|-------------|
| `USE_MIRROR` | `auto` | Mirror source selection:<br>- `auto`: Auto-detect (uses mirror if Google unavailable)<br>- `true`: Force China mirrors (Aliyun + npm mirror)<br>- `false`: Force official sources |

## Build Process Details

### Step 1: Tauri Builder Stage
1. **Install System Dependencies**
   - pkg-config, libssl-dev, libsqlite3-dev
   - GTK3, WebKit2GTK, AppIndicator, librsvg

2. **Install Node.js 20**
   - Via NodeSource repository
   - Configure npm registry (mirror if enabled)

3. **Install Rust Dependencies**
   - Configure Cargo registry (mirror if enabled)
   - Set `CARGO_HTTP_MULTIPLEXING=false` for compatibility

4. **Build Application**
   ```bash
   npm install        # Install frontend dependencies
   npm run tauri build # Build frontend + backend
   ```

### Step 2: Runtime Stage
1. **Install Runtime Dependencies Only**
   - libssl3, libsqlite3-0, ca-certificates
   - GTK3, WebKit2GTK runtime libraries

2. **Copy Build Artifacts**
   - Binary: `/app/src-tauri/target/release/antigravity_tools`
   - Frontend: `/app/dist/`

3. **Configure Runtime**
   - Environment: `RUST_LOG=info`, `PORT=8046`
   - Entrypoint: `/app/antigravity-tools --headless`

## Testing the Build

### 1. Build the Image
```bash
docker build -t antigravity-manager:test -f docker/Dockerfile .
```

### 2. Run the Container
```bash
docker run -d \
  --name antigravity-test \
  -p 8046:8046 \
  -e API_KEY=test-key-123 \
  -v ~/.antigravity_tools:/root/.antigravity_tools \
  antigravity-manager:test
```

### 3. Verify
```bash
# Check logs
docker logs antigravity-test

# Access Web UI
curl http://localhost:8046

# Check health
docker ps | grep antigravity
```

### 4. Cleanup
```bash
docker stop antigravity-test
docker rm antigravity-test
docker rmi antigravity-manager:test
```

## Troubleshooting

### Build Fails: "Connection timeout"
**Solution**: Use China mirrors
```bash
docker build --build-arg USE_MIRROR=true -f docker/Dockerfile .
```

### Build Fails: "npm install error"
**Solution**: Clear npm cache and retry
```bash
docker build --no-cache -f docker/Dockerfile .
```

### Build Fails: "cargo build error"
**Solution**: Check Rust version and dependencies
```bash
# Verify Rust toolchain in container
docker run --rm rust:1.85-slim rustc --version
```

### Binary Not Found in Runtime
**Solution**: Verify binary name matches in `tauri.conf.json`
```bash
# Check binary name
grep -A 5 '"bundle"' src-tauri/tauri.conf.json
```

## Build Performance

| Region | Build Time (avg) | Recommended USE_MIRROR |
|--------|------------------|------------------------|
| China Mainland | 15-20 min | `true` or `auto` |
| International | 10-15 min | `false` or `auto` |

### Tips for Faster Builds
1. **Use BuildKit** (Docker 18.09+)
   ```bash
   DOCKER_BUILDKIT=1 docker build -f docker/Dockerfile .
   ```

2. **Use Layer Caching**
   - Separate dependency installation from source code copy
   - Already optimized in the Dockerfile

3. **Parallel Builds**
   ```bash
   # Build with more CPU cores
   docker build --build-arg CARGO_BUILD_JOBS=8 -f docker/Dockerfile .
   ```

## Advanced: Multi-Platform Builds

Build for multiple architectures (requires Docker Buildx):

```bash
# Create builder
docker buildx create --name multiplatform --use

# Build for AMD64 + ARM64
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t antigravity-manager:latest \
  -f docker/Dockerfile \
  --push \
  .
```

## CI/CD Integration

### GitHub Actions Example
```yaml
- name: Build Docker Image
  run: |
    docker build \
      --build-arg USE_MIRROR=${{ secrets.USE_CHINA_MIRROR }} \
      -t ${{ secrets.DOCKER_REGISTRY }}/antigravity-manager:${{ github.sha }} \
      -f docker/Dockerfile .
```

### GitLab CI Example
```yaml
build:
  script:
    - docker build --build-arg USE_MIRROR=true -t $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA -f docker/Dockerfile .
```

## Related Files

- [`Dockerfile`](./Dockerfile) - Main build configuration
- [`docker-compose.yml`](./docker-compose.yml) - Compose deployment
- [`README.md`](./README.md) - Deployment documentation
