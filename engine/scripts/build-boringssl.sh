#!/usr/bin/env bash
# Clone and build BoringSSL for aarch64-linux-android, matching the tree that
# built the keymint/inject binaries in use (the build host's ~/.cargo/boringssl).
# The configure flags below are taken from that tree's build/CMakeCache.txt.
#
# Usage: scripts/build-boringssl.sh [DEST_DIR]
#   DEST_DIR (or $BORINGSSL_DIR)  checkout location; build output goes to DEST_DIR/build.
#                                 Must not exist yet or be an existing checkout.
# Environment overrides:
#   BORINGSSL_COMMIT  default ed9209cff8b5a2fbc6c10050f29b3355048bda85
#   BORINGSSL_URL     default https://boringssl.googlesource.com/boringssl
#   ANDROID_HOME      default /opt/android/sdk
#   ANDROID_NDK_ROOT  default $ANDROID_HOME/ndk/29.0.14206865
#   CMAKE             default $ANDROID_HOME/cmake/4.1.2/bin/cmake (falls back to cmake on PATH)
#   BINDGEN           default bindgen on PATH (the reference build used bindgen-cli 0.73.2:
#                     cargo install bindgen-cli --version 0.73.2 --locked)
#
# Afterwards build-android.sh takes BORINGSSL_DIR=DEST_DIR.
set -euo pipefail

DEST="${1:-${BORINGSSL_DIR:-}}"
if [ -z "$DEST" ]; then
    echo "usage: $0 DEST_DIR   (or set BORINGSSL_DIR)" >&2
    exit 2
fi

BORINGSSL_COMMIT="${BORINGSSL_COMMIT:-ed9209cff8b5a2fbc6c10050f29b3355048bda85}"
BORINGSSL_URL="${BORINGSSL_URL:-https://boringssl.googlesource.com/boringssl}"
ANDROID_HOME="${ANDROID_HOME:-/opt/android/sdk}"
ANDROID_NDK_ROOT="${ANDROID_NDK_ROOT:-$ANDROID_HOME/ndk/29.0.14206865}"
CMAKE="${CMAKE:-$ANDROID_HOME/cmake/4.1.2/bin/cmake}"
[ -x "$CMAKE" ] || CMAKE="$(command -v cmake)"
[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"
BINDGEN="${BINDGEN:-$(command -v bindgen || true)}"

[ -f "$ANDROID_NDK_ROOT/build/cmake/android.toolchain.cmake" ] || {
    echo "NDK not found: $ANDROID_NDK_ROOT" >&2; exit 1; }
[ -n "$BINDGEN" ] && [ -x "$BINDGEN" ] || {
    echo "bindgen not found (cargo install bindgen-cli --version 0.73.2 --locked)" >&2; exit 1; }
command -v ninja >/dev/null || { echo "ninja not found" >&2; exit 1; }
"$BINDGEN" --version | grep -q '0\.73\.2' || echo "warning: bindgen is not 0.73.2: $("$BINDGEN" --version)" >&2

if [ ! -d "$DEST/.git" ]; then
    if [ -e "$DEST" ] && [ -n "$(ls -A "$DEST" 2>/dev/null)" ]; then
        echo "$DEST exists and is not a git checkout; refusing to touch it" >&2
        exit 1
    fi
    git clone "$BORINGSSL_URL" "$DEST"
fi
git -C "$DEST" fetch --quiet origin "$BORINGSSL_COMMIT" 2>/dev/null || true
git -C "$DEST" -c advice.detachedHead=false checkout "$BORINGSSL_COMMIT"
[ "$(git -C "$DEST" rev-parse HEAD)" = "$BORINGSSL_COMMIT" ] || {
    echo "checkout of $BORINGSSL_COMMIT failed" >&2; exit 1; }

"$CMAKE" -S "$DEST" -B "$DEST/build" -G Ninja \
    -DCMAKE_TOOLCHAIN_FILE="$ANDROID_NDK_ROOT/build/cmake/android.toolchain.cmake" \
    -DANDROID_ABI=arm64-v8a \
    -DANDROID_PLATFORM=android-21 \
    -DANDROID_STL=c++_shared \
    -DCMAKE_SYSROOT="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/sysroot/" \
    -DCMAKE_C_FLAGS=-mno-outline-atomics \
    -DCMAKE_CXX_FLAGS=-mno-outline-atomics \
    -DCMAKE_INSTALL_PREFIX="$DEST/install" \
    -DRUST_BINDINGS=aarch64-linux-android \
    -DBINDGEN_EXECUTABLE="$BINDGEN"
ninja -C "$DEST/build"

test -f "$DEST/build/libcrypto.a"
test -f "$DEST/build/rust/bssl-sys/wrapper_aarch64-linux-android.rs"
echo "BoringSSL $BORINGSSL_COMMIT built in $DEST/build"
