FROM rustembedded/cross:armv7-unknown-linux-gnueabihf-0.2.1

RUN dpkg --add-architecture armhf && \
    apt update && \
    apt --assume-yes dist-upgrade && \
    apt install --assume-yes libsqlite3-dev:armhf