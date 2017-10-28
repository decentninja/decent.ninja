FROM rust:1.21.0
WORKDIR /usr/src/decentninja2
COPY . .
ENV ROCKET_ENV production
RUN cargo install
CMD ["decentninja2"]
