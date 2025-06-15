#!/bin/bash

# Docker build script for r-fubon-neo
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
IMAGE_NAME="r-fubon-neo"
TAG="latest"
BUILD_TYPE="production"
PUSH=false
REGISTRY=""

# Function to display usage
usage() {
    echo "Usage: $0 [OPTIONS]"
    echo "Options:"
    echo "  -t, --tag TAG          Tag for the image (default: latest)"
    echo "  -d, --dev              Build development image"
    echo "  -s, --static           Build static linked image"
    echo "  --distroless           Build distroless static image"
    echo "  -p, --push             Push image to registry after build"
    echo "  -r, --registry URL     Registry URL for pushing"
    echo "  -h, --help             Display this help message"
    echo ""
    echo "Examples:"
    echo "  $0 -t v2.2.3                    # Build production image with tag v2.2.3"
    echo "  $0 -d -t dev                    # Build development image"
    echo "  $0 -s -t static                 # Build static linked image"
    echo "  $0 --distroless -t distroless   # Build distroless static image"
    echo "  $0 -t v2.2.3 -p -r my-registry.com  # Build and push to registry"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -t|--tag)
            TAG="$2"
            shift 2
            ;;
        -d|--dev)
            BUILD_TYPE="development"
            shift
            ;;
        -s|--static)
            BUILD_TYPE="static"
            shift
            ;;
        --distroless)
            BUILD_TYPE="distroless"
            shift
            ;;
        -p|--push)
            PUSH=true
            shift
            ;;
        -r|--registry)
            REGISTRY="$2"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "Unknown option $1"
            usage
            exit 1
            ;;
    esac
done

# Set full image name
if [[ -n "$REGISTRY" ]]; then
    FULL_IMAGE_NAME="$REGISTRY/$IMAGE_NAME:$TAG"
else
    FULL_IMAGE_NAME="$IMAGE_NAME:$TAG"
fi

echo -e "${YELLOW}Building Docker image...${NC}"
echo "Image: $FULL_IMAGE_NAME"
echo "Build type: $BUILD_TYPE"
echo ""

# Build the image
case "$BUILD_TYPE" in
    "development")
        echo -e "${YELLOW}Building development image...${NC}"
        docker build -f Dockerfile.dev --target development -t "$FULL_IMAGE_NAME" .
        ;;
    "static")
        echo -e "${YELLOW}Building static linked image...${NC}"
        docker build -f Dockerfile.static --target static -t "$FULL_IMAGE_NAME" .
        ;;
    "distroless")
        echo -e "${YELLOW}Building distroless static image...${NC}"
        docker build -f Dockerfile.static --target distroless -t "$FULL_IMAGE_NAME" .
        ;;
    *)
        echo -e "${YELLOW}Building production image...${NC}"
        docker build -f Dockerfile -t "$FULL_IMAGE_NAME" .
        ;;
esac

# Check if build was successful
if [[ $? -eq 0 ]]; then
    echo -e "${GREEN}✓ Build successful!${NC}"
    
    # Display image info
    echo ""
    docker images "$FULL_IMAGE_NAME"
    
    # Push if requested
    if [[ "$PUSH" == true ]]; then
        if [[ -z "$REGISTRY" ]]; then
            echo -e "${RED}Error: Registry URL is required for pushing${NC}"
            exit 1
        fi
        
        echo -e "${YELLOW}Pushing image to registry...${NC}"
        docker push "$FULL_IMAGE_NAME"
        
        if [[ $? -eq 0 ]]; then
            echo -e "${GREEN}✓ Push successful!${NC}"
        else
            echo -e "${RED}✗ Push failed!${NC}"
            exit 1
        fi
    fi
    
    echo ""
    echo -e "${GREEN}Build completed successfully!${NC}"
    case "$BUILD_TYPE" in
        "static")
            echo "Run with: docker run --rm $FULL_IMAGE_NAME"
            echo "Image size optimized with static linking (scratch base)"
            ;;
        "distroless")
            echo "Run with: docker run --rm $FULL_IMAGE_NAME"
            echo "Image size optimized with static linking (distroless base)"
            ;;
        *)
            echo "Run with: docker run --rm $FULL_IMAGE_NAME"
            ;;
    esac
    
else
    echo -e "${RED}✗ Build failed!${NC}"
    exit 1
fi