FROM frolvlad/alpine-glibc:glibc-2.34
WORKDIR /app

COPY target/release/far-memory /app/app

ENTRYPOINT ["/app/app"]