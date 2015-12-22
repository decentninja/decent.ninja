FROM nginx:latest
EXPOSE 80
COPY . /var/www
CMD ["nginx"]
