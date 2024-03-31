FROM ros:humble-ros-base

ARG DEBIAN_FRONTEND=noninteractive
RUN apt update && apt -y upgrade && apt -y install \
 git \
 curl \
 libclang-dev \
 python3-pip \
 python3-vcstool

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain 1.74.0 -y  && \
 export PATH=/root/.cargo/bin:$PATH && \
 cargo install cargo-ament-build  && \
 pip3 install git+https://github.com/colcon/colcon-cargo.git && \
 pip3 install git+https://github.com/colcon/colcon-ros-cargo.git

ENV PATH=/root/.cargo/bin:$PATH

RUN mkdir -p /workspace && echo "Did you forget to mount the repository into the Docker container?" > /workspace/HELLO.txt
WORKDIR /robot-car
