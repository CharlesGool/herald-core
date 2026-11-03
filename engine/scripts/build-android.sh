#!/usr/bin/env bash
# Build the two binaries herald ships, keymint and inject, for aarch64-linux-android.
# Replaces a former ad-hoc build script. Run from anywhere; it builds the engine
# tree that contains this script.
#
# Environment overrides (defaults match the reference build host):
#   ANDROID_HOME         /opt/android/sdk
#   ANDROID_NDK_ROOT     $ANDROID_HOME/ndk/29.0.14206865
#   BORINGSSL_DIR        $HOME/.cargo/boringssl   (checkout from scripts/build-boringssl.sh)
#   BORINGSSL_BUILD_DIR  $BORINGSSL_DIR/build
#   ANDROID_API_LEVEL    26
#   HERALD_SOURCE_COMMIT source commit recorded in BUILD-INFO (no .git on the build host,
#                        so the caller passes it; "unknown" if unset)
#
# Output: target/aarch64-linux-android/release/{keymint,inject} and BUILD-INFO
# next to target/.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"

export ANDROID_HOME="${ANDROID_HOME:-/opt/android/sdk}"
ANDROID_NDK_ROOT="${ANDROID_NDK_ROOT:-$ANDROID_HOME/ndk/29.0.14206865}"
BORINGSSL_DIR="${BORINGSSL_DIR:-$HOME/.cargo/boringssl}"
export BORINGSSL_BUILD_DIR="${BORINGSSL_BUILD_DIR:-$BORINGSSL_DIR/build}"
ANDROID_API_LEVEL="${ANDROID_API_LEVEL:-26}"
HERALD_SOURCE_COMMIT="${HERALD_SOURCE_COMMIT:-unknown}"
TARGET=aarch64-linux-android
OUT="target/$TARGET/release"

for f in "$ANDROID_NDK_ROOT/build/cmake/android.toolchain.cmake" \
         "$BORINGSSL_DIR/rust/bssl-sys/Cargo.toml" \
         "$BORINGSSL_BUILD_DIR/libcrypto.a"; do
    [ -f "$f" ] || { echo "missing: $f" >&2; exit 1; }
done

python3 scripts/setup_cargo_config.py --ndk-root "$ANDROID_NDK_ROOT" \
    --api-level "$ANDROID_API_LEVEL" --force
printf '\n[patch.crates-io]\nbssl-sys = { path = "%s" }\n' \
    "$BORINGSSL_DIR/rust/bssl-sys" >> .cargo/config.toml

# keymint embeds the absolute path of the generated OUT_DIR/aidl.rs (panic locations),
# so the checkout location would change the binary. Map it to a fixed prefix. Arrays
# from --config are appended to the generated target rustflags, not replacing them.
REMAP="--config=target.$TARGET.rustflags=[\"--remap-path-prefix=$ROOT=/herald-engine\"]"

cargo build --locked --release --target "$TARGET" "$REMAP" -p injector --bin inject
cargo build --locked --release --target "$TARGET" "$REMAP" --bin keymint

boringssl_commit="$(git -C "$BORINGSSL_DIR" rev-parse HEAD 2>/dev/null || echo unknown)"
{
    echo "source_commit: $HERALD_SOURCE_COMMIT"
    echo "built_at: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "rustc: $(rustc -V)"
    echo "boringssl_commit: $boringssl_commit"
    echo "ndk: $ANDROID_NDK_ROOT"
    echo "sha256:"
    (cd "$OUT" && sha256sum keymint inject | sed 's/^/  /')
} > BUILD-INFO
cat BUILD-INFO
