# Vendored crates

## wgpu-hal 30.0.1 (patched)

Copy of wgpu-hal 30.0.1 from crates.io (MIT OR Apache-2.0, licence files inside) with one change in
`src/dx12/instance.rs` (marked "AnyPlay patch") and the re-export in `src/dx12/mod.rs`:
`wgpu_hal::dx12::set_only_adapter(Some((luid_high, luid_low)))` makes `enumerate_adapters` expose just
that adapter. Without it wgpu creates a D3D12 device on every adapter to read its capabilities, and on a
hybrid laptop that wakes the discrete GPU for about 3 seconds before the first frame
(docs/DESIGN.md 3.7, "Start-up"). Hooked in through `[patch.crates-io]` in the root `Cargo.toml`.

Drop the patch when wgpu offers an adapter filter (or when we propose one upstream and it lands):
delete this folder and the `[patch.crates-io]` entry.
