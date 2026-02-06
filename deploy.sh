#!/bin/bash

# Definition of colors for logs
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

LOG_FILE="deploy.log"

log() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1" | tee -a "$LOG_FILE"
}

warn() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] WARNING:${NC} $1" | tee -a "$LOG_FILE"
}

error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ERROR:${NC} $1" | tee -a "$LOG_FILE"
}

log "Starting deployment of Antigravity Manager..."

# 1. Check and Clean Port 8046
PORT=8046
log "Checking port $PORT..."

# Try to find the PID using the port
if command -v lsof >/dev/null 2>&1; then
    PID=$(lsof -t -i:$PORT)
elif command -v netstat >/dev/null 2>&1; then
    PID=$(netstat -nlp | grep :$PORT | awk '{print $7}' | cut -d'/' -f1)
elif command -v ss >/dev/null 2>&1; then
    PID=$(ss -lptn 'sport = :'$PORT | grep pid= | sed 's/.*pid=\([0-9]*\).*/\1/')
fi

if [ ! -z "$PID" ]; then
    warn "Port $PORT is used by process PID: $PID. Killing process..."
    kill -9 $PID
    sleep 2
    log "Process $PID killed."
else
    log "Port $PORT is free."
fi

# 2. Stop and Remove Existing Container
CONTAINER_NAME="antigravity-manager"
if [ "$(docker ps -aq -f name=^/${CONTAINER_NAME}$)" ]; then
    log "Stopping existing container: $CONTAINER_NAME..."
    docker stop $CONTAINER_NAME
    log "Removing existing container: $CONTAINER_NAME..."
    docker rm $CONTAINER_NAME
else
    log "No existing container named $CONTAINER_NAME found."
fi

# 3. Pull Latest Changes (Optional, assuming script is run after git pull)
# log "Pulling latest git changes..."
# git pull origin main

# 4. Start Docker Compose
log "Starting services with Docker Compose..."
if [ -f "docker/docker-compose.yml" ]; then
    docker compose -f docker/docker-compose.yml up -d --remove-orphans --build
    
    if [ $? -eq 0 ]; then
        log "Docker Compose started successfully."
    else
        error "Failed to start Docker Compose."
        exit 1
    fi
else
    error "docker/docker-compose.yml not found!"
    exit 1
fi

# 5. Check Health/Logs
log "Waiting for service to initialize (5 seconds)..."
sleep 5
log "Showing recent logs:"
docker logs --tail 20 $CONTAINER_NAME

log "Deployment finished successfully!"
