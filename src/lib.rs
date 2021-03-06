pub mod call;

/// Define a function that BYOND can call into.
///
/// The list of arguments is always `&str`, and the code block must return an `impl AsRef<[u8]>`.
/// If you want finer control over the arguments and return value,
/// consider using `from_byond_args` and `return_to_byond()` directly.
/// # Examples
/// ```
/// # use byond::byond;
/// // Define a function called "test", taking two arguments.
/// byond!(thing: one, two; {
///     format!("{} + {}", one, two)
/// });
///
/// # fn main(){}
/// ```
///
/// The above code can be called with the following DM code:
///
/// ```dm
/// // Produces "a + b" without quotes.
/// world.log << call("test.dll", "thing")("a", "b")
/// ```
///
/// # Panics
/// Panics if the amount of arguments supplied by BYOND is too small.
/// Note that extra arguments are ignored.
/// Also panics if a NUL byte is attempted to be returned.
#[macro_export]
macro_rules! byond {
    ( $n:ident ; $c:block ) => {
        #[no_mangle]
        pub unsafe extern "C" fn $n (__n: i32, __v: *const *const std::os::raw::c_char) -> *const std::os::raw::c_char {
            $crate::call::return_to_byond((|| $c)()).unwrap()
        }
    };
    ( $n:ident : $( $p:ident ),* ; $c:block ) => {
        #[no_mangle]
        pub unsafe extern "C" fn $n (__n: i32, __v: *const *const std::os::raw::c_char) -> *const std::os::raw::c_char {
            let __args = $crate::call::from_byond_args(__n, __v);

            let mut __count = 0;
            $(
                let $p: &str = &__args[__count];
                __count += 1;
            )*

            let ret = (|| $c)();

            $crate::call::return_to_byond(ret).unwrap()
        }
    };
}
