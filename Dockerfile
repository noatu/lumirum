FROM node:22-alpine AS web-builder
WORKDIR /app
COPY website/package*.json .
RUN npm ci
COPY website/ .
RUN npm run build

FROM caddy:alpine
COPY --from=web-builder /app/dist /srv
COPY Caddyfile /etc/caddy/Caddyfile
