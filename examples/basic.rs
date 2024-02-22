use owi::{assert, i64};

pub fn main() {
    let x = i64();

    // Prove `x + 1 > x`
    assert(x + 1 > x);
}
