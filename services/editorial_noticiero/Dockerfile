FROM nginxinc/nginx-unprivileged:1.27-alpine

ENV CORE_URL=http://10.20.30.10:8080

COPY public/ /usr/share/nginx/html/
COPY default.conf.template /etc/nginx/templates/default.conf.template

EXPOSE 8080