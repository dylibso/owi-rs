#![no_std]
//! A crate for generating WebAssembly modules that can be executed using the
//! [Owi](https://github.com/OCamlPro/owi) symbolic runtime

mod sys {
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
    ($a: ident, $x:ident, ($($n:ident : $t:ty),*) $(-> $r:ty)?, $doc:expr) => {
        #[doc=$doc]
        pub fn $a($($n:$t),*) $(-> $r)? {
            unsafe { sys::$x($($n),*) }
        }
    };
}

wrap!(assume, assume, (x: bool), "Add an assumption to the verifier");
wrap!(assert, assert, (x: bool), "Add an assertion to the verifier");
wrap!(i8, i8_symbol, () -> i8, "Create an `i8` symbolic value");
wrap!(i32, i32_symbol, () -> i32, "Create an `i32` symbolic value");
wrap!(i64, i64_symbol, () -> i64, "Create an `i64` symbolc value");
wrap!(f32, f32_symbol, () -> f32, "Create an `f32` symbolic value");
wrap!(f64, f64_symbol, () -> f64, "Create an `f64` symbolic value");

#[doc = "Create a `u8` symbolic value"]
pub fn u8() -> u8 {
    i8() as u8
}

#[doc = "Create a `u32` symbolic value"]
pub fn u32() -> u32 {
    i32() as u32
}

#[doc = "Create a `u64` symbolic value"]
pub fn u64() -> u64 {
    i64() as u64
}

/// `main!` is used to setup the `_start` function and, when compiling for
/// `wasm32-unknown-unknown` it also sets up the panic handler.
#[macro_export]
macro_rules! main {
    ($b:block) => {
        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        #[panic_handler]
        fn panic(_info: &core::panic::PanicInfo) -> ! {
            core::arch::wasm32::unreachable()
        }

        #[no_mangle]
        pub unsafe fn _start() {
            $b
        }
    };
}
