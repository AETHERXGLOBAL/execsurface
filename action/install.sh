#!/usr/bin/env bash
set -euo pipefail

if [[ "$(uname -s)" != "Linux" || "$(uname -m)" != "x86_64" ]]; then
  echo "::error title=ExecSurface::Public alpha currently supports Linux x86_64 only"
  exit 2
fi

if [[ -d "$GITHUB_ACTION_PATH/.git" ]]; then
  if ! command -v cargo >/dev/null 2>&1; then
    echo "::error title=ExecSurface::Cargo is required only for local Action development"
    exit 2
  fi
  target_dir="$RUNNER_TEMP/execsurface-action-target"
  cargo build --locked --release \
    --manifest-path "$GITHUB_ACTION_PATH/Cargo.toml" \
    -p execsurface --bin execsurface \
    --target-dir "$target_dir"
  binary="$target_dir/release/execsurface"
else
  for tool in curl tar sha256sum; do
    if ! command -v "$tool" >/dev/null 2>&1; then
      echo "::error title=ExecSurface::required tool missing: $tool"
      exit 2
    fi
  done

  release_tag="$(tr -d '[:space:]' < "$GITHUB_ACTION_PATH/action/release-tag.txt")"
  if [[ ! "$release_tag" =~ ^v[0-9]+\.[0-9]+\.[0-9]+-[0-9A-Za-z.-]+$ ]]; then
    echo "::error title=ExecSurface::invalid pinned release tag: $release_tag"
    exit 2
  fi

  target="x86_64-unknown-linux-gnu"
  asset="execsurface-${release_tag}-${target}.tar.gz"
  base="https://github.com/AETHERXGLOBAL/execsurface/releases/download/${release_tag}"
  install_dir="$RUNNER_TEMP/execsurface-action-${release_tag}"
  mkdir -p "$install_dir"

  curl --fail --location --silent --show-error \
    --output "$install_dir/$asset" "$base/$asset"
  curl --fail --location --silent --show-error \
    --output "$install_dir/$asset.sha256" "$base/$asset.sha256"

  (
    cd "$install_dir"
    sha256sum -c "$asset.sha256"
  )

  tar -xzf "$install_dir/$asset" -C "$install_dir"
  binary="$install_dir/execsurface-${release_tag}-${target}/execsurface"
  expected_version="${release_tag#v}"
  actual_version="$("$binary" --version)"
  if [[ "$actual_version" != "execsurface $expected_version" ]]; then
    echo "::error title=ExecSurface::release identity mismatch: $actual_version"
    exit 2
  fi
fi

test -x "$binary"
echo "EXECSURFACE_ACTION_BIN=$binary" >> "$GITHUB_ENV"
