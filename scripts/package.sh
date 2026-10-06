set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

NAME="BumpkinLegends-Miniapp"
BINARY_NAME="bumpkin-legends-bot"
VERSION="$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)"
STAGE="dist/$NAME"
ZIP_BASE="${NAME}-${VERSION}"

WITH_BINARY=0
[[ "${1:-}" == "--binary" ]] && WITH_BINARY=1

echo "==> Bumpkin Legends Miniapp packager"
echo "    version : $VERSION"
echo "    root    : $NAME/"
echo "    mode    : $( [[ $WITH_BINARY -eq 1 ]] && echo 'source + binary' || echo 'source only' )"

BIN_PATH="target/release/${BINARY_NAME}"
if [[ $WITH_BINARY -eq 1 ]]; then
    echo "==> cargo build --release"
    cargo build --release
    if [[ ! -f "$BIN_PATH" ]]; then
        echo "!! release binary missing at $BIN_PATH" >&2
        exit 1
    fi
    echo "    binary: $(du -h "$BIN_PATH" | cut -f1)"
fi

echo "==> staging files"
rm -rf "$STAGE"
mkdir -p "$STAGE"

copy() { # copy <path> [dest-subdir]
    local src="$1" dest="${2:-}"
    if [[ ! -e "$src" ]]; then
        echo "    !! missing, skipped: $src" >&2
        return 0
    fi
    mkdir -p "$STAGE/$dest"
    cp -r "$src" "$STAGE/$dest/"
    echo "    + ${dest:+$dest/}$(basename "$src")"
}

copy Cargo.toml
copy Cargo.lock
copy build.rs
copy Makefile
copy run.sh
copy config.json
copy data.txt
copy proxy.txt
copy README.md
copy LICENSE
copy .gitignore
copy assets
copy src
copy scripts
copy .github

if [[ $WITH_BINARY -eq 1 ]]; then
    mkdir -p "$STAGE/bin"
    cp "$BIN_PATH" "$STAGE/bin/${BINARY_NAME}"
    chmod +x "$STAGE/bin/${BINARY_NAME}"
    echo "    + bin/${BINARY_NAME}"
fi

chmod +x "$STAGE/run.sh" 2>/dev/null || true
chmod +x "$STAGE/scripts/"*.sh 2>/dev/null || true

echo "==> writing archive"
mkdir -p dist
if [[ $WITH_BINARY -eq 1 ]]; then
    OUT="dist/${ZIP_BASE}-binary.zip"
else
    OUT="dist/${ZIP_BASE}-source.zip"
fi
rm -f "$OUT"

if command -v zip >/dev/null 2>&1; then
    ( cd dist && zip -qr "$(basename "$OUT")" "$NAME" \
        -x "$NAME/target/*" -x "$NAME/dist/*" -x "$NAME/*.log" \
        -x '*__pycache__*' -x '*.pyc' -x '*/.DS_Store' )
else
    echo "    zip not found, falling back to python"
    python3 - "$STAGE" "$OUT" <<'PY'
import os, sys, zipfile
stage, out = sys.argv[1], sys.argv[2]
base = os.path.dirname(stage)
with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as z:
    for root, _dirs, files in os.walk(stage):
        for f in files:
            full = os.path.join(root, f)
            z.write(full, os.path.relpath(full, base))
PY
fi

rm -rf "$STAGE"

SIZE="$(du -h "$OUT" | cut -f1)"
MD5="$(md5sum "$OUT" | cut -d' ' -f1)"
echo
echo "==> done"
echo "    archive : $OUT"
echo "    size    : $SIZE"
echo "    md5     : $MD5"
