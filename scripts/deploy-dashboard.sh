#!/bin/bash

# NEAR TEE Oracle - Dashboard Deployment Script
# Deploy the monitoring dashboard to various platforms

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

PLATFORM=${1:-vercel}
ENV=${2:-production}

echo -e "${GREEN}NEAR TEE Oracle - Dashboard Deployment${NC}"
echo "========================================"
echo "Platform: $PLATFORM"
echo "Environment: $ENV"
echo ""

cd dashboard

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo -e "${RED}Error: Node.js not found${NC}"
    echo "Install from: https://nodejs.org/"
    exit 1
fi

# Install dependencies
echo -e "${YELLOW}Installing dependencies...${NC}"
npm install

# Build the dashboard
echo -e "${YELLOW}Building dashboard...${NC}"
npm run build

echo -e "${GREEN}Build successful${NC}"
echo ""

# Deploy based on platform
case $PLATFORM in
    vercel)
        echo -e "${YELLOW}Deploying to Vercel...${NC}"
        if ! command -v vercel &> /dev/null; then
            echo "Installing Vercel CLI..."
            npm install -g vercel
        fi
        
        if [ "$ENV" == "production" ]; then
            vercel --prod
        else
            vercel
        fi
        ;;
        
    netlify)
        echo -e "${YELLOW}Deploying to Netlify...${NC}"
        if ! command -v netlify &> /dev/null; then
            echo "Installing Netlify CLI..."
            npm install -g netlify-cli
        fi
        
        if [ "$ENV" == "production" ]; then
            netlify deploy --prod --dir=out
        else
            netlify deploy --dir=out
        fi
        ;;
        
    docker)
        echo -e "${YELLOW}Building Docker image...${NC}"
        
        # Create Dockerfile if not exists
        if [ ! -f "Dockerfile" ]; then
            cat > Dockerfile << 'EOF'
FROM node:18-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM node:18-alpine
WORKDIR /app
COPY --from=builder /app/.next ./.next
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package.json ./package.json
COPY --from=builder /app/public ./public

EXPOSE 3000
CMD ["npm", "start"]
EOF
        fi
        
        docker build -t near-tee-oracle-dashboard:$ENV .
        
        if [ "$ENV" == "production" ]; then
            echo -e "${GREEN}Docker image built: near-tee-oracle-dashboard:production${NC}"
            echo ""
            echo "To run:"
            echo "docker run -p 3000:3000 near-tee-oracle-dashboard:production"
        else
            docker run -d -p 3000:3000 --name oracle-dashboard near-tee-oracle-dashboard:$ENV
            echo -e "${GREEN}Dashboard running at http://localhost:3000${NC}"
        fi
        ;;
        
    static)
        echo -e "${YELLOW}Generating static export...${NC}"
        npm run build
        
        echo -e "${GREEN}Static files generated in ./out/${NC}"
        echo ""
        echo "Deploy the ./out directory to any static hosting:"
        echo "- AWS S3 + CloudFront"
        echo "- GitHub Pages"
        echo "- Cloudflare Pages"
        echo "- Any static host"
        ;;
        
    *)
        echo -e "${RED}Unknown platform: $PLATFORM${NC}"
        echo ""
        echo "Supported platforms:"
        echo "  vercel   - Deploy to Vercel"
        echo "  netlify  - Deploy to Netlify"
        echo "  docker   - Build Docker image"
        echo "  static   - Generate static files"
        echo ""
        echo "Usage: ./deploy-dashboard.sh [platform] [environment]"
        echo "Example: ./deploy-dashboard.sh vercel production"
        exit 1
        ;;
esac

echo ""
echo -e "${GREEN}Deployment complete!${NC}"

