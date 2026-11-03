#!/usr/bin/env bash
# herald-core 的检查:脚本语法 + 借 TB321FU-GSI-Mods 的 herald 回归测试跑一遍本仓库的 patch/。
#   用法: scripts/check.sh            (make check)
#   MODS_DIR=<Mods 仓库>  默认 ../TB321FU-GSI-Mods;不在就只做语法检查
#   STRICT=1              Mods 仓库不在时算失败(CI 用)
# 不改 Mods 仓库:把它的 toolkit/patches/devices 拷进临时目录,把本仓库的 patch/ 同步进拷贝里再跑测试。
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
MODS_DIR=${MODS_DIR:-$ROOT/../TB321FU-GSI-Mods}
fail=0

echo "== 语法"
SH=$(command -v mksh || command -v sh)   # 真机的 sh 是 mksh
for f in "$ROOT"/patch/files/herald-start.sh "$ROOT"/patch/files/heraldctl; do
    if "$SH" -n "$f"; then echo "  通过  ${f#$ROOT/}"; else echo "  失败  ${f#$ROOT/}"; fail=1; fi
done
for f in "$ROOT"/patch/apply.sh "$ROOT"/scripts/*.sh "$ROOT"/engine/scripts/*.sh; do
    [[ -f $f ]] || continue
    if bash -n "$f"; then echo "  通过  ${f#$ROOT/}"; else echo "  失败  ${f#$ROOT/}"; fail=1; fi
done

echo "== 回归测试(Mods 仓库的 herald 测试)"
if [[ ! -d $MODS_DIR/toolkit/tests ]]; then
    echo "  跳过:没有 Mods 仓库 $MODS_DIR(设 MODS_DIR 指定)"
    [[ -n ${STRICT:-} ]] && fail=1
    exit $fail
fi
T=$(mktemp -d); trap 'rm -rf "$T"' EXIT
mkdir -p "$T/mods"
cp -r "$MODS_DIR"/toolkit "$MODS_DIR"/patches "$MODS_DIR"/devices "$T/mods/"
# prefix/full 测试从 Mods 上一级的 legacy/ 取私有 payload 的 seed;有就链过去,没有就跳过这两个
LEGACY=$MODS_DIR/../legacy
[[ -d $LEGACY ]] && ln -s "$(cd "$LEGACY" && pwd)" "$T/legacy"
HERALD_CORE_DIR=$ROOT bash "$T/mods/toolkit/sync-herald-core.sh" >/dev/null

run() {   # run <说明> <命令…>:输出只留最后一行,失败时整段打出来
    local name=$1 out; shift
    if out=$("$@" 2>&1); then echo "  通过  $name: ${out##*$'\n'}"
    else echo "  失败  $name"; echo "$out" | tail -n 30; fail=1; fi
}
run 协议 bash "$T/mods/toolkit/tests/test-herald-protocol.sh" "$T/mods/devices/tb321fu-a15/device.conf"
if [[ -d $T/legacy/gsi/strong/herald-payload/seed ]]; then
    run 开机前缀 bash "$T/mods/toolkit/tests/test-herald-start-prefix.sh"
    run 完整开机 bash "$T/mods/toolkit/tests/test-herald-start-full.sh" --repo
else
    echo "  跳过  开机前缀/完整开机:没有私有 payload seed($LEGACY/gsi/strong/herald-payload/seed)"
fi
exit $fail
