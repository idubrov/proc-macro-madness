#!/bin/sh

pushd madness-types
cargo clean

echo "benching direct handling of attribute macro"
cargo build --features direct
touch src/gen.rs && cargo build --features direct
touch src/gen.rs && cargo build --features direct
touch src/gen.rs && cargo build --features direct

echo "testing desugaring to derive"
cargo clean
cargo build --features desugar
touch src/gen.rs && cargo build --features desugar
touch src/gen.rs && cargo build --features desugar
touch src/gen.rs && cargo build --features desugar
