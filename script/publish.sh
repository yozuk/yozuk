#!/bin/sh

set -e

NEXT_TAG=$1
LAST_TAG=$(git describe --tags --abbrev=0)

updateCrate() {
    DIFF=$(git diff --name-only dev..$LAST_TAG $1)
    
    if [[ -n "$DIFF" ]]; then
        echo "$1: Changed"
        sed -i -E "0,/version/ s/version = \"[.0-9]+\"/version = \"${NEXT_TAG#v}\"/" $1/Cargo.toml
        sed -i -E "s/$2 = \"[.0-9]+\"/$2 = \"${NEXT_TAG#v}\"/" */Cargo.toml */*/Cargo.toml
        sed -i -E "s/$2 = \{ version = \"[.0-9]+\"/$2 = { version = \"${NEXT_TAG#v}\"/" */Cargo.toml */*/Cargo.toml
        taplo fmt $1/Cargo.toml
    else
        echo "$1: Unchanged"
    fi
}

bumpCrate() {
    updateCrate $1 $2
    if [[ $(git diff --stat) != '' ]]; then
        cargo fmt --check
        cargo clippy --all-features
        git commit -a -m "bump $2 $NEXT_TAG"
    fi
}

publishCrate() {
    updateCrate $1 $2
    if [[ $(git diff --stat) != '' ]]; then
        cargo fmt --check
        cargo clippy --all-features
        git commit -a -m "publish $2 $NEXT_TAG"
        for i in {1..3}; do cargo publish -p $2 && break || sleep 10; done
    fi
}

publishTestCrate() {
    updateCrate $1 $2
    if [[ $(git diff --stat) != '' ]]; then
        cargo fmt --check
        cargo clippy --all-features
        cargo nextest run --all-features
        git commit -a -m "publish $2 $NEXT_TAG"
        for i in {1..3}; do cargo publish -p $2 && break || sleep 10; done
    fi
}

publishCrate "yozuk-sdk" "yozuk-sdk"
publishCrate "helpers/filetype" "yozuk-helper-filetype"
publishCrate "helpers/platform" "yozuk-helper-platform"
publishCrate "helpers/english" "yozuk-helper-english"
publishCrate "helpers/encoding" "yozuk-helper-encoding"
publishCrate "yozuk-model" "yozuk-model"
publishCrate "skillset" "yozuk-core-skillset"
publishTestCrate "yozuk" "yozuk"
publishTestCrate "zuk" "zuk"

sed -i -E "0,/version/ s/version = \"[.0-9]+\"/version = \"${NEXT_TAG#v}\"/" wapm.toml
git commit -a -m "publish zuk-wasi $NEXT_TAG"

sed -i -E "0,/version/ s/\"version\": \"[.0-9]+\"/\"version\": \"${NEXT_TAG#v}\"/" yozuk-wasm/package.json
(cd yozuk-wasm && npm run build && npm publish)
git commit -a -m "publish yozuk-wasm $NEXT_TAG"

git tag $NEXT_TAG