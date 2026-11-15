# Etapa 1: compilar la página con Node
FROM node:20-alpine AS build
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

# Etapa 2: servir solo los archivos compilados, sin root
FROM nginxinc/nginx-unprivileged:1.27-alpine
ENV CORE_URL=http://10.20.30.10:8080
COPY --from=build /app/dist /usr/share/nginx/html
COPY default.conf.template /etc/nginx/templates/default.conf.template
EXPOSE 8080