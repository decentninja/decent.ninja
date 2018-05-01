FROM dolphm/ubuntu-latest-rust-nightly
WORKDIR /usr/src/decentninja2
COPY . .
ENV ROCKET_ENV production
RUN ~/.cargo/bin/cargo install
CMD ["decentninja2"]
