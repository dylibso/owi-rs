use owi::{assert, i64_symbol};

pub fn main() {
    let x = i64_symbol();

    // Prove `x + 1 > x`
    assert(x + 1 > x);
}
