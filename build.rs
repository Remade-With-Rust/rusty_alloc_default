//! Present only because Cargo requires a build script on any package that sets
//! `links`. There is nothing native to probe: the `links = "rusty_alloc_global"`
//! key exists so Cargo rejects a dependency graph holding two allocator
//! installers, which it decides at resolve time from the manifests alone.

fn main() {
    // Nothing to detect and nothing to emit. Re-run only when this file changes,
    // so an empty script never invalidates the build cache.
    println!("cargo:rerun-if-changed=build.rs");
}
