// Integration test for SigmaOS-arch-anisotropic-filtering-kernel
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use arch_anisotropic_filtering_kernel as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
