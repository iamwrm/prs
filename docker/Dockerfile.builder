FROM centos:7 as builder

RUN \
    yum update -y \
    && yum install -y epel-release \
    && yum update -y \
    && yum install -y curl git make gcc sudo

RUN \
    groupadd -g 1000 u1000 \
    && groupadd -g 1001 u1001 \
    && useradd -m -u 1000 -g 1000 -s /bin/bash u1000 \
    && usermod -aG u1001 u1000 \
    && echo "u1000 ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

USER u1000

RUN \ 
    sudo ls /root && echo "sudo works"

WORKDIR /home/u1000

RUN \
    curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && mkdir -p /home/u1000/.cargo/{git,registry,registry/index,registry/cache}

