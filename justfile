set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# Setup a new day
@new DAY:
    cargo new --vcs none day{{DAY}}
    echo "/target" > day{{DAY}}/.gitignore

# Get the output of a day
@run DAY:
    cd day{{DAY}} && cargo run -q

# Run clippy on a day
@check DAY:
    cd day{{DAY}} && cargo clippy