FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

ARG TARGETARCH

COPY build/linux-${TARGETARCH}/archipelagio /usr/local/bin/archipelagio
RUN ln -s archipelagio /usr/local/bin/aio

ENTRYPOINT ["archipelagio"]
