FROM rust:alpine as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

FROM alpine
COPY --from=builder /usr/local/cargo/bin/app /usr/local/bin/gompei25

CMD ["gompei25"]