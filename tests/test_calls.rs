#[macro_use]
extern crate byond;

use byond::call::{test_byond_call, test_byond_call_args};

byond!(test_args: a, b, c; {
    format!("{} + {} + {}", a, b, c)
});

byond!(test_noargs; {
    "Hello, World!"
});

byond!(test_return: a; {
    if a == "1" {
        return "hi!";
    } else {
        return "bye!";
    }
});

#[test]
fn test_calls() {
    assert_eq!(test_byond_call_args(test_args, &["A", "B", "C"]),
               "A + B + C");
    assert_eq!(test_byond_call(test_noargs), "Hello, World!");
    assert_eq!(test_byond_call_args(test_return, &["1"]), "hi!");
    assert_eq!(test_byond_call_args(test_return, &["0"]), "bye!");
}

#[test]
#[should_panic]
fn test_panic() {
    test_byond_call_args(test_args, &[]);
}

#[test]
#[should_panic]
fn test_panic_noargs() {
    test_byond_call(test_args);
}