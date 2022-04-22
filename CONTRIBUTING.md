# Contributing to Yozuk

## Pull Requests

Before sending a request, please make sure that the following commands return no error.

```bash
cargo fmt --check
cargo clippy --all-features
cargo test --all-features
```

## Adding a new feature

New features should meet the following criteria as much as possible:

- Easy-to-use.
- Available on low-end devices.
- Compatible with WASM.

### Discouraged features

We may reject feature proposals with the following issues:

- Needing network access (e.g. currency converter, DNS lookup or port scanner).
- Needing additional permissions (e.g. camera access or root access).
- Affecting user environments (e.g. editing file or registry).
- Consuming a large amount of computation resources (e.g. brute-force search).
- Depending on a large dataset (e.g. language dictionary or rainbow table).
- Depending on a specific platform or hardware. 
- Making a heavy impact on binary size, startup time or compile time.
- Conflicting with existing features.
- Violating [Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).

## Code of Conduct

https://www.contributor-covenant.org/version/2/1/code_of_conduct/
