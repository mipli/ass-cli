#!/bin/bash

cargo_out_dir() {
  # This works by finding the most recent stamp file
  target_dir="$1"
  find "$target_dir" -name ass-cli-stamp -print0 \
    | xargs -0 ls -t \
    | head -n1 \
    | xargs dirname
}

main() {
  cargo build --target "$TARGET" --release

  local tmpdir="$(mktemp -d)"
  local name="ass-cli-${TARGET}"
  local staging="$tmpdir/$name"
  mkdir -p "$staging"
  local out_dir="$(pwd)/deployment"
  mkdir -p "$out_dir"

  local cargo_out_dir="$(cargo_out_dir "target/$TARGET")"

  cp "target/$TARGET/release/ass-cli" "$staging/ass-cli"
  "strip" "$staging/ass-cli"

  cp {README.md,LICENSE-MIT} "$staging/"

  (cd "$tmpdir" && tar czf "$out_dir/$name.tar.gz" "$name")
  rm -rf "$tmpdir"
}

main
