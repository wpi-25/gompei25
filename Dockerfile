FROM rust as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

FROM alpine
COPY --from=builder /usr/local/cargo/bin/gompei25 /usr/local/bin/gompei25

CMD ["gompei25"]