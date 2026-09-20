# webspace

[![crates.io](https://img.shields.io/crates/v/webspace.svg)](https://crates.io/crates/webspace)
[![docs.rs](https://docs.rs/webspace/badge.svg)](https://docs.rs/webspace)

Web and WebAssembly-facing utilities for the [userspace.party](https://userspace.party) ecosystem.

## Role

`webspace` is the browser-facing edge of the crate family. It keeps the ecosystem's `no_std`-first shape while exposing WebAssembly-specific integrations only when targeting `wasm32`.

The current source includes:

- HTML abstractions;
- DOM helpers;
- canvas support;
- browser audio/canvas/element bindings on WebAssembly targets;
- integration with `humans` for human-facing visual and audible concepts.

On `wasm32`, the crate intentionally uses `wasm-bindgen` and `web-sys`. These are platform bindings rather than general-purpose runtime dependencies.

## Use

```bash
cargo add webspace
```

For browser builds, compile for a WebAssembly target such as:

```bash
rustup target add wasm32-unknown-unknown
cargo check --target wasm32-unknown-unknown
```

## Ecosystem

- Ecosystem: https://userspace.party
- Crate homepage: https://userspace.party/webspace
- API documentation: https://docs.rs/webspace
- crates.io: https://crates.io/crates/webspace
- Source: https://github.com/ze-gois/rust_webspace
- Workspace hub: https://github.com/ze-gois/rust_userspace_hub

`webspace` is independently published but participates in the coordinated userspace release line.

## Status

Experimental. The browser surface is evolving together with `humans` and the rest of the dependency-light stack.

## License

See [LICENSE](LICENSE).
