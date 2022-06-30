FROM ubuntu:focal

LABEL maintainer="PurplProto - https://github.com/PurplProto/docker-arma3-server"

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN dpkg --add-architecture i386 \
  && apt-get update \
  && apt-get install -y --no-install-recommends --no-install-suggests \
  ca-certificates \
  lib32gcc-s1 \
  lib32stdc++6 \
  libcurl4 \
  wget \
  && apt-get remove --purge -y \
  && apt-get clean autoclean \
  && apt-get autoremove -y \
  && rm -rf /var/lib/apt/lists/* \
  && mkdir -p /steamcmd \
  && wget -qO- 'https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz' | tar zxf - -C /steamcmd

WORKDIR /arma3

VOLUME /steamcmd

EXPOSE 2302/udp
EXPOSE 2303/udp
EXPOSE 2304/udp
EXPOSE 2305/udp
EXPOSE 2306/udp

ENV EXTRA_PACKAGES=
ENV ARMA_BINARY=./arma3server
ENV ARMA_CONFIG=main.cfg
ENV ARMA_EXTRA_PARAMS=
ENV ARMA_PROFILE=main
ENV ARMA_WORLD=empty
ENV ARMA_LIMITFPS=1000
ENV ARMA_HEADLESS_CLIENTS=0
ENV MODS_RUN_FROM_ARMA_DIR=true
ENV STEAM_BRANCH=public
ENV STEAM_BRANCH_PASSWORD=
ENV STEAM_SKIP_INSTALL=false

STOPSIGNAL SIGINT

CMD ["bash", "-c", "echo 'Hello world'"]
