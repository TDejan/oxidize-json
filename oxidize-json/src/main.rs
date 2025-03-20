// The binary crate's entry point (contains `main`)

mod core;

// A path is a way of naming items (like functions, structs, modules) in Rust. Paths can be absolute (from the root of the crate) or relative (from the current module).
// use core::generator; brings the generator module into scope.
use core::generator as gen; // A module inside the crate for organizing code

fn main() {
    gen::parse_json();
    gen::generate_schema();
}
