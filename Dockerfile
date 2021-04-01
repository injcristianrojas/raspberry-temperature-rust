# Build first: docker build -t cross-arm:0.1 .

FROM rustembedded/cross:armv7-unknown-linux-gnueabihf-0.2.1

RUN dpkg --add-architecture armhf
RUN apt-get update && apt-get install -y --no-install-recommends libsqlite3-dev:armhf=3.11.0-1ubuntu1.5 && apt-get clean && rm -rf /var/lib/apt/lists/*