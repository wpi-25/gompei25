FROM ekidd/rust-musl-builder as builder

ADD --chown=rust:rust . ./

RUN cargo build --release

FROM alpine

RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/gompei25 \
    /usr/local/bin/gompei25

ENTRYPOINT [ "gompei25" ]