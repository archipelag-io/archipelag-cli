FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

ARG TARGETARCH

COPY build/linux-${TARGETARCH}/archipelag /usr/local/bin/archipelag

ENTRYPOINT ["archipelag"]
