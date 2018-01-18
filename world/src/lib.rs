
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate util;

pub mod player;
pub mod tile;
pub mod map;
pub mod load;
pub mod print;
pub mod colour;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
