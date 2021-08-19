#! /usr/bin/env bash

if [[ ! -f "Cargo.lock" ]]; then
    cargo build
fi
echo "+----------------------------------------------------------"
echo "|    Running kcov"
echo "+----------------------------------------------------------"

cargo kcov --verbose -- --limits=95,98 --exclude-pattern=mock_,_bindings.rs --include-pattern=$PWD/src/lib --exclude-region=KCOV_SKIP_START:KCOV_SKIP_END --exclude-line=KCOV_SKIP_LINE

echo "+----------------------------------------------------------"
echo "|    Kcov done: target/cov/index.html"
echo "+----------------------------------------------------------"

