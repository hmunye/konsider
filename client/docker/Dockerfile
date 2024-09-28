## -------------------------------------------------------------------------------------
FROM node:20-alpine AS deps

WORKDIR /app

COPY package*.json ./

RUN npm ci
## -------------------------------------------------------------------------------------
FROM node:20-alpine AS builder

WORKDIR /app

COPY . .

COPY --from=deps /app/node_modules ./node_modules

RUN npx next telemetry disable

RUN npm run build
## -------------------------------------------------------------------------------------
FROM node:20-alpine AS runtime

WORKDIR /app

RUN addgroup -S client && adduser -S client-user -G client

COPY --from=builder /app/.next/standalone /app/
COPY --from=builder /app/public /app/public
COPY --from=builder /app/.next/static ./.next/static 
COPY --from=builder /app/.env.production /app/.env.production

RUN chown -R client-user:client /app

USER client-user

ENV PORT=3000

CMD ["node", "server.js"]
