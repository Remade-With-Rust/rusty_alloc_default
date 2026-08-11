# rusty_alloc_default

[![Remade With Rust](https://img.shields.io/badge/Remade%20With-Rust-000?logo=rust&logoColor=fff)](https://github.com/remade-with-rust)
[![By Mata Network](https://img.shields.io/badge/by-Mata%20Network-5b2be0)](https://www.mata.network)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE-MIT)
![MSRV: 1.73](https://img.shields.io/badge/MSRV-1.73-informational)

> Tiny seam that installs [`rusty_alloc`](https://github.com/Remade-With-Rust/rusty_alloc)
> as the process `#[global_allocator]`. Used as the **default** `rusty-alloc`
> feature of `rusty_symbols`, `rusty_tokens`, and `rusty_a11y` so those crates
> can each default-on without fighting when combined (one link, one allocator).

```toml
rusty_alloc_default = "0.1"
# hardened:
# rusty_alloc_default = { version = "0.1", features = ["secure"] }
```

Apps that already install an allocator (e.g. via `mata-alloc`) should not
depend on this crate -- set `default-features = false` on the Remade UI crates.

## License

MIT -- [LICENSE-MIT](LICENSE-MIT).
