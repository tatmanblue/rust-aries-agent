# may need to go ubuntu route once we start using indy
# FROM ubuntu:18.04
FROM rust:latest

ARG uid=1000

RUN apt-get update && \
    apt-get install -y \
      pkg-config \
      libssl-dev \
      curl \
      libsqlite3-dev \
      cmake \
      python3-pip \
      debhelper \
      devscripts \
      libncursesw5-dev \
      libzmq3-dev \
      libsodium-dev \
      wget \
      git \
      vim

RUN pip3 install -U \
	pip \
	twine \
	plumbum==1.6.7 six==1.12.0 \
	deb-pkg-tools

RUN useradd -ms /bin/bash -u $uid aries-agent
USER aries-agent

WORKDIR /home/aries-agent