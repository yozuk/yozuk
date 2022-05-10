set positional-arguments

alias r := run
alias c := check

@run *args='': 
    cargo r -- $@
    
check:
    cargo fmt --check
    cargo clippy --all-features
    cargo nextest run --all-features
