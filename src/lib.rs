#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Installs [`rusty_alloc`](https://github.com/Remade-With-Rust/rusty_alloc) as
//! the process-wide [`GlobalAlloc`](core::alloc::GlobalAlloc).
//!
//! Remade UI crates (`rusty_symbols`, `rusty_tokens`, `rusty_a11y`) enable this
//! via their default `rusty-alloc` feature. Depending on several of those crates
//! with defaults on is safe: Cargo links **this** crate once, so there is still
//! exactly one `#[global_allocator]`.
//!
//! Opt out in a consumer by disabling the parent crate's default features, or
//! by not depending on this crate at all when the app installs its own
//! allocator (e.g. `mata-alloc`).

use rusty_alloc_api::RustyAlloc;

#[global_allocator]
static GLOBAL: RustyAlloc = RustyAlloc;

/// Always `true` -- linking this crate installs the allocator.
pub const fn enabled() -> bool {
    true
}

/// Whether the hardened `secure` profile is compiled in.
pub const fn secure_enabled() -> bool {
    cfg!(feature = "secure")
}
