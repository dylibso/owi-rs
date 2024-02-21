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
    ($x:ident, ($($n:ident : $t:ty),*) $(-> $r:ty)?) => {
        pub fn $x($($n:$t),*) $(-> $r)? {
            unsafe { sys::$x($($n),*) }
        }
    };
}

wrap!(assume, (x: bool));
wrap!(assert, (x: bool));
wrap!(i8_symbol, () -> i8);
wrap!(i32_symbol, () -> i32);
wrap!(i64_symbol, () -> i64);
wrap!(f32_symbol, () -> f32);
wrap!(f64_symbol, () -> f64);

pub fn u8_symbol() -> u8 {
    i8_symbol() as u8
}

pub fn u32_symbol() -> u32 {
    i32_symbol() as u32
}

pub fn u64_symbol() -> u64 {
    i64_symbol() as u64
}
