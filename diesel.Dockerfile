FROM frolvlad/alpine-rust as builder

RUN apk add --no-cache postgresql-dev && \
    cargo install diesel_cli --no-default-features --features postgres

FROM alpine:latest
COPY --from=builder /root/.cargo/bin/diesel /bin/diesel

RUN apk add --no-cache postgresql-dev libgcc

ENTRYPOINT ["/bin/diesel"]
