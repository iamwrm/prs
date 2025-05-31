#!/bin/bash

set -euo pipefail

readonly ARCH="amd64"
readonly UPX_VERSION="4.2.1"
readonly DOWNLOAD_URL="https://github.com/upx/upx/releases/download/v${UPX_VERSION}/upx-${UPX_VERSION}-${ARCH}_linux.tar.xz"
readonly UPX_ARCHIVE="upx-${UPX_VERSION}-${ARCH}_linux.tar.xz"
readonly DOWNLOAD_DIR="$(pwd)/local_data"
readonly BIN_DIR="${DOWNLOAD_DIR}/bin"

mkdir -p "${BIN_DIR}"

if [ ! -f "${DOWNLOAD_DIR}/${UPX_ARCHIVE}" ]; then
    echo "📥 Downloading UPX ${UPX_VERSION}..."
    curl -sL "${DOWNLOAD_URL}" -o "${DOWNLOAD_DIR}/${UPX_ARCHIVE}"
fi

echo "📦 Extracting UPX..."
tar -xf "${DOWNLOAD_DIR}/${UPX_ARCHIVE}" -C "${DOWNLOAD_DIR}" 2>/dev/null

cp "${DOWNLOAD_DIR}/upx-${UPX_VERSION}-${ARCH}_linux/upx" "${BIN_DIR}/"
chmod +x "${BIN_DIR}/upx"

export PATH="${BIN_DIR}:$PATH"

echo "✅ UPX $(upx --version | head -n1) ready"
