FROM shepmaster/rust-nightly
WORKDIR /rust/decentninja2
ADD . .
ENV ROCKET_ENV production
EXPOSE 80
RUN rustc --version
RUN cargo install
CMD ["decentninja2"]