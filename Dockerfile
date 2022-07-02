FROM nginx:latest

COPY ./nginx.conf /etc/nginx/nginx.conf

COPY web-app/static/. ./static