FROM rustembedded/cross:i686-unknown-linux-gnu-0.2.0

RUN dpkg --add-architecture i386 && \
    apt-get update && \
    apt-get install -y libx11-dev:i386 libxcb-render0-dev:i386 libxcb-shape0-dev:i386 libxcb-xfixes0-dev:i386 python3:i386
