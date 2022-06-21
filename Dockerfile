# Build first: podman build -t cross-arm:0.1 .

FROM ghcr.io/cross-rs/armv7-unknown-linux-gnueabihf:edge

RUN dpkg --add-architecture armhf && \
apt-get update && \
apt-get install -y --no-install-recommends libsqlite3-dev:armhf=3.11.0-1ubuntu1.5 && \
apt-get clean && \
rm -rf /var/lib/apt/lists/*