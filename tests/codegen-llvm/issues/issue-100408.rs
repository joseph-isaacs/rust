//@ compile-flags: -Copt-level=3 -Z mir-opt-level=4
#![crate_type = "lib"]

use std::num::NonZeroI32;

#[no_mangle]
pub fn foo(x: NonZeroI32) -> i32 {
    // ensure that NonZero division has no divide-by-zero check
    // CHECK-LABEL: @foo(
    // CHECK-NOT: icmp eq i32
    // CHECK: %_0 = sdiv i32 33, %x
    // CHECK-NOT: panic
    (|x: &NonZeroI32| 33 / x.get())(&x)
}
