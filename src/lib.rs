pub mod cluster;
pub mod spatial;
pub mod utils;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[cfg(test)]
// mod tests {
//     use crate::cluster::polyhedron;

//     #[test]
//     fn test() {
//         let tet = polyhedron::tetrahedron::new(3);
//         print!("{}", tet);
//     }

//     #[test]
//     fn test2() {
//         let ico = polyhedron::icosahedron::new(4);
//         print!("{}", ico)
//     }
// }
