FROM rust
WORKDIR /usr/src/decentninja2
copy . .
run rustup default nightly
ENV ROCKET_ENV production
EXPOSE 80
RUN rustc --version
RUN cargo install
CMD ["decentninja2"]