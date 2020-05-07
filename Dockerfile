FROM debian:stable-slim
WORKDIR /usr/src/decentninja2
copy target/release/decentninja2 .
copy static static
copy Rocket.toml .
copy templates templates
copy content.json .
ENV ROCKET_ENV production
EXPOSE 80
CMD ["./decentninja2"]
