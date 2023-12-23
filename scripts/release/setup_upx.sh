#!/bin/bash

set -ueo pipefail

ARCH=amd64
UPX_VERSION="4.2.1"
DOWNLOAD_URL="https://github.com/upx/upx/releases/download/v${UPX_VERSION}/upx-${UPX_VERSION}-${ARCH}_linux.tar.xz"
UPX_ARCHIVE_NAME="upx-${UPX_VERSION}-${ARCH}_linux.tar.xz"
DOWNLOAD_DIR=$(pwd)/local_data


mkdir -p ${DOWNLOAD_DIR}/bin

if [ ! -f ${DOWNLOAD_DIR}/${UPX_ARCHIVE_NAME} ]; then
    curl -L ${DOWNLOAD_URL} -o ${DOWNLOAD_DIR}/${UPX_ARCHIVE_NAME} 
fi

tar -xvf ${DOWNLOAD_DIR}/${UPX_ARCHIVE_NAME} -C ${DOWNLOAD_DIR}

cp ${DOWNLOAD_DIR}/upx-${UPX_VERSION}-${ARCH}_linux/upx ${DOWNLOAD_DIR}/bin
chmod +x ${DOWNLOAD_DIR}/bin/upx


export PATH=${DOWNLOAD_DIR}/bin:$PATH

upx --version
