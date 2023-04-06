//!
//! ```
//! use anyhow::Result;
//! pub fn get_ip(opt: Option<(&str, &str)>) -> Result<&'static String> {
//!    maybe_static!(opt, String, |(addr, port)| format!("{addr}:{port}"))
//! }
//!
//! fn main() {
//!     println!("{}", get_ip(Some(("hello", "world"))).unwrap());
//!     println!("{}", get_ip(None).unwrap());
//! }
//!```

#[macro_export]

/// ```
/// use anyhow::Result;
/// pub fn get_ip(opt: Option<(&str, &str)>) -> Result<&'static String> {
///    maybe_static!(opt, String, |(addr, port)| format!("{addr}:{port}"))
/// }
///
/// fn main() {
///     println!("{}", get_ip(Some(("hello", "world"))).unwrap());
///     println!("{}", get_ip(None).unwrap());
/// }
/// ```
macro_rules! maybe_static {
    ($arg:expr, $t:ty, $fn:expr) => {{
        use std::{mem::MaybeUninit, sync::Once};

        static mut LAZY: (MaybeUninit<$t>, Once) = (MaybeUninit::uninit(), Once::new());
        unsafe {
            static mut ERR: bool = false;
            LAZY.1.call_once(|| match $arg {
                Some(arg) => {
                    LAZY.0.write($fn(arg));
                }
                _ => ERR = true,
            });
            match ERR {
                _ => Ok(&*LAZY.0.as_ptr()), // you're sure that it's initialized here
                true => Err("error: maybe static require a filled option at the fist call"),
            }
        }
    }};
}

#[macro_export]
macro_rules! maybe_static_unsafe {
    ($arg:expr, $t:ty, $fn:expr) => {{
        use std::{mem::MaybeUninit, sync::Once};

        static mut LAZY: (MaybeUninit<$t>, Once) = (MaybeUninit::uninit(), Once::new());
        unsafe {
            static mut ERR: bool = false;
            LAZY.1.call_once(|| match $arg {
                Some(arg) => {
                    LAZY.0.write($fn(arg));
                }
                _ => ERR = true,
            });
            match ERR {
                _ => Ok(&mut *LAZY.0.as_mut_ptr()), // you're sure that it's initialized here
                true => Err("error: maybe static require a filled option at the fist call"),
            }
        }
    }};

    ($t:ty, $fn:expr) => {{
        use std::{mem::MaybeUninit, sync::Once};

        static mut LAZY: (MaybeUninit<$t>, Once) = (MaybeUninit::uninit(), Once::new());
        unsafe {
            static mut ERR: bool = false;
            LAZY.1.call_once(|| {
                LAZY.0.write($fn());
            });
            &mut *LAZY.0.as_mut_ptr() // you're sure that it's initialized here
        }
    }};
}
