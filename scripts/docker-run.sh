#!/bin/bash

# Docker run script for r-fubon-neo
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
IMAGE_NAME="r-fubon-neo:latest"
API_KEY=""
SECRET_KEY=""
COMMAND="version"
INTERACTIVE=false
DETACH=false
CONTAINER_NAME=""

# Function to display usage
usage() {
    echo "Usage: $0 [OPTIONS] [COMMAND]"
    echo "Options:"
    echo "  -i, --image IMAGE      Docker image to run (default: r-fubon-neo:latest)"
    echo "  -k, --api-key KEY      API key for authentication"
    echo "  -s, --secret-key KEY   Secret key for authentication"
    echo "  -t, --interactive      Run in interactive mode"
    echo "  -d, --detach           Run in detached mode"
    echo "  -n, --name NAME        Container name"
    echo "  -h, --help             Display this help message"
    echo ""
    echo "Commands:"
    echo "  version                Show version information"
    echo "  test                   Test SDK connection"
    echo "  market-data            Initialize market data"
    echo ""
    echo "Examples:"
    echo "  $0 version                                    # Show version"
    echo "  $0 -k YOUR_KEY -s YOUR_SECRET test          # Test connection"
    echo "  $0 -k YOUR_KEY -s YOUR_SECRET market-data   # Initialize market data"
    echo "  $0 -t -n fubon-dev version                  # Interactive mode with custom name"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -i|--image)
            IMAGE_NAME="$2"
            shift 2
            ;;
        -k|--api-key)
            API_KEY="$2"
            shift 2
            ;;
        -s|--secret-key)
            SECRET_KEY="$2"
            shift 2
            ;;
        -t|--interactive)
            INTERACTIVE=true
            shift
            ;;
        -d|--detach)
            DETACH=true
            shift
            ;;
        -n|--name)
            CONTAINER_NAME="$2"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        -*)
            echo "Unknown option $1"
            usage
            exit 1
            ;;
        *)
            COMMAND="$1"
            shift
            ;;
    esac
done

# Build docker run command
DOCKER_CMD="docker run --rm"

# Add interactive flag if requested
if [[ "$INTERACTIVE" == true ]]; then
    DOCKER_CMD="$DOCKER_CMD -it"
fi

# Add detach flag if requested
if [[ "$DETACH" == true ]]; then
    DOCKER_CMD="$DOCKER_CMD -d"
fi

# Add container name if provided
if [[ -n "$CONTAINER_NAME" ]]; then
    DOCKER_CMD="$DOCKER_CMD --name $CONTAINER_NAME"
fi

# Add environment variables for credentials
if [[ -n "$API_KEY" ]]; then
    DOCKER_CMD="$DOCKER_CMD -e FUBON_API_KEY=$API_KEY"
fi

if [[ -n "$SECRET_KEY" ]]; then
    DOCKER_CMD="$DOCKER_CMD -e FUBON_SECRET_KEY=$SECRET_KEY"
fi

# Add volume mounts for config and logs
DOCKER_CMD="$DOCKER_CMD -v $(pwd)/config:/app/config:ro"
DOCKER_CMD="$DOCKER_CMD -v $(pwd)/logs:/app/logs"

# Add image name
DOCKER_CMD="$DOCKER_CMD $IMAGE_NAME"

# Build command arguments
if [[ "$COMMAND" == "test" || "$COMMAND" == "market-data" ]]; then
    if [[ -z "$API_KEY" || -z "$SECRET_KEY" ]]; then
        echo -e "${RED}Error: API key and secret key are required for '$COMMAND' command${NC}"
        echo "Use -k/--api-key and -s/--secret-key options"
        exit 1
    fi
    DOCKER_CMD="$DOCKER_CMD --api-key $API_KEY --secret-key $SECRET_KEY $COMMAND"
else
    DOCKER_CMD="$DOCKER_CMD $COMMAND"
fi

# Display the command being run
echo -e "${YELLOW}Running Docker container...${NC}"
echo "Image: $IMAGE_NAME"
echo "Command: $COMMAND"
if [[ -n "$API_KEY" ]]; then
    echo "API Key: ${API_KEY:0:8}..."
fi
echo ""

# Create directories if they don't exist
mkdir -p config logs

# Run the command
echo -e "${GREEN}Executing: $DOCKER_CMD${NC}"
eval $DOCKER_CMD

# Check exit code
if [[ $? -eq 0 ]]; then
    echo -e "${GREEN}✓ Command completed successfully!${NC}"
else
    echo -e "${RED}✗ Command failed!${NC}"
    exit 1
fi