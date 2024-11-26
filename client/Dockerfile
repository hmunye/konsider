FROM node:22-alpine AS builder

WORKDIR /app

COPY package*.json ./

RUN npm ci

COPY . .

RUN npm run build
#------------------------------------------------------------------------------
FROM node:22-alpine AS runtime

WORKDIR /app

RUN addgroup -S client && adduser -S client-user -G client

COPY --from=builder /app/build ./build
COPY --from=builder /app/.env.production ./.env.production
COPY --from=builder /app/package.json ./package.json
COPY --from=builder /app/node_modules ./node_modules

RUN chown -R client-user:client /app

USER client-user

ENV PORT=3080
ENV NODE_ENV=production

ENTRYPOINT ["/bin/sh", "-c", "node --env-file=.env.production build"]
