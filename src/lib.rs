#![no_std]

pub mod sys {
    #[link(wasm_import_module = "symbolic")]
    extern "C" {
        pub fn i8_symbol() -> i8;
        pub fn i32_symbol() -> i32;
        pub fn i64_symbol() -> i64;
        pub fn f32_symbol() -> f32;
        pub fn f64_symbol() -> f64;
        pub fn assume(x: bool);
        pub fn assert(x: bool);
    }
}

macro_rules! wrap {
    ($x:ident, ($($n:ident : $t:ty),*) $(-> $r:ty)?, $doc:expr) => {
        #[doc=$doc]
        pub fn $x($($n:$t),*) $(-> $r)? {
            unsafe { sys::$x($($n),*) }
        }
    };
}

wrap!(assume, (x: bool), "Add an assumption to the verifier");
wrap!(assert, (x: bool), "Add an assertion to the verifier");
wrap!(i8_symbol, () -> i8, "Create an `i8` symbolic value");
wrap!(i32_symbol, () -> i32, "Create an `i32` symbolic value");
wrap!(i64_symbol, () -> i64, "Create an `i64` symbolc value");
wrap!(f32_symbol, () -> f32, "Create an `f32` symbolic value");
wrap!(f64_symbol, () -> f64, "Create an `f64` symbolic value");

#[doc = "Create a `u8` symbolic value"]
pub fn u8_symbol() -> u8 {
    i8_symbol() as u8
}

#[doc = "Create a `u32` symbolic value"]
pub fn u32_symbol() -> u32 {
    i32_symbol() as u32
}

#[doc = "Create a `u64` symbolic value"]
pub fn u64_symbol() -> u64 {
    i64_symbol() as u64
}
