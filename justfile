alias r := run
run:
    cargo run -- example/test.csv

alias wb := watch-build
watch-build:
    watchexec -w src -- "reset && cargo build"

