#!/bin/bash

set -euo pipefail

export DOCKER_BUILDKIT=1
export DOCKER=docker

echo "🔨 Building Rust binary with cross-compilation..."

# Build the builder image and compile the binary
${DOCKER} build -t rust_builder -f docker/Dockerfile.builder .

${DOCKER} run --rm --init -v ${PWD}:/app rust_builder \
	bash -c "source /root/.bashrc && cd /app && cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.17"

echo "✅ Binary compiled successfully"

# Build runtime image
${DOCKER} build -t prs -f docker/Dockerfile.runtime .

# Fix permissions
sudo find . -type d -exec chmod 0755 {} \;
sudo find . -type f -exec chmod 0644 {} \;

# Extract binary from Docker image
${DOCKER} run --rm -v ${PWD}:/host prs \
	bash -c "cp /usr/local/bin/prs /host && chown ${HOST_UID:-1000}:${HOST_GID:-1000} /host/prs"

sudo chmod +x ./prs

echo "📊 Checking GLIBC requirements:"
objdump -T ./prs | grep GLIBC | sed 's/.*GLIBC_\([.0-9]*\).*/\1/g' | sort -Vu

echo "🧪 Testing binary:"
${DOCKER} run --rm prs /usr/local/bin/prs -p top10-mem
./prs -p top10-mem

echo "📦 Compressing binary with UPX..."
bash scripts/release/setup_upx.sh

export PATH=${PWD}/local_data/bin:$PATH

echo "Before compression: $(ls -lah ./prs | awk '{print $5}')"
upx --ultra-brute ./prs
echo "After compression: $(ls -lah ./prs | awk '{print $5}')"

echo "🎉 Build completed successfully!"