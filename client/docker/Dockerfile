## -------------------------------------------------------------------------------------
FROM node:20-alpine AS deps

WORKDIR /app

# Copy package.json and package-lock.json to the working directory
COPY package*.json ./

# Install dependencies
RUN npm ci

## -------------------------------------------------------------------------------------
FROM node:20-alpine AS builder

WORKDIR /app

# Copy all application files
COPY . .

# Copy node_modules from the deps stage
COPY --from=deps /app/node_modules ./node_modules

RUN npx next telemetry disable

# Build the application
RUN npm run build

## -------------------------------------------------------------------------------------
FROM node:20-alpine AS runtime

WORKDIR /app


RUN addgroup -S client && adduser -S client-user -G client

# Copy the built application and necessary files
COPY --from=builder /app/.next/standalone /app/
COPY --from=builder /app/public /app/public
COPY --from=builder /app/.next/static ./.next/static 
COPY --from=builder /app/.env.production /app/.env.production

RUN chown -R client-user:client /app

USER client-user

# Set environment variable
ENV PORT=3000

# Start the application
CMD ["node", "server.js"]
