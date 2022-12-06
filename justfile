set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# Setup a new day
@new DAY:
    cargo new --vcs none day{{DAY}}
    echo "/target" > day{{DAY}}/.gitignore

# Get the output of a day
@run DAY:
    cargo run -q --manifest-path ./day{{DAY}}/Cargo.toml

# Run clippy on a day
@check DAY:
    cargo clippy --manifest-path ./day{{DAY}}/Cargo.toml