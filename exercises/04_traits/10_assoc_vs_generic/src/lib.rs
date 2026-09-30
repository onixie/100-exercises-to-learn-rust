// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

use num_traits::{Num, NumRef, RefNum, ToPrimitive};
use std::{
    any::Any,
    borrow::Borrow,
    ops::{Deref, Mul, Sub},
    process::Output,
};

// Move BOTH the exponent type U and the base target type T into the trait definition
pub trait Power<T> {
    type Output;
    fn power(self, idx: T) -> Self::Output;
}

pub trait Pow<T, RHS> {
    fn pow2(self, rhs: RHS) -> Self;
}

trait IsRefOrSelf<U>: Borrow<U> {}
impl<T> IsRefOrSelf<T> for &'_ T {}
impl<T> IsRefOrSelf<T> for T where
    T: Default /* To unbreak the ambiguity since reference won't implement Default */
{
}

// Without the above trick, boilerplate like below will be needed.
// impl IsRef<u16, u16> for u16 {}
// impl IsRef<u32, u32> for u32 {}

impl<T, RHS> Pow<T, RHS> for u32
where
    T: ToPrimitive,
    RHS: IsRefOrSelf<T>,
{
    fn pow2(self, idx: RHS) -> Self {
        let mut res = 1;
        for _ in 0..(idx.borrow().to_usize().expect("idx must be non-negative")) {
            res *= self;
        }
        res
        // if idx.borrow().is_zero() {
        //     1
        // } else if idx.borrow().is_one() {
        //     self
        // } else {
        //     self * self.pow2(*idx.borrow() - T::one())
        // }
    }
}

// impl<U, T> Power<U, T> for u32
// where
//     // 1. T is the concrete underlying numeric type (e.g., u32, u16)
//     T: Num + NumRef + Sub<Output = U> + Copy,
//     for<'a> &'a T: Sub<Output = U>,

//     // 2. Bound U so that it must explicitly be converted or dereferenced into T
//     // This allows both `u16` and `&u16` to map directly to `T = u16`
//     U: std::ops::Deref<Target = T> + Copy,
// {
//     type Output = Self;

//     fn power(self, idx: U) -> Self::Output {
//         // Since U implements Deref<Target = T>, *idx uniquely and cleanly gives us T
//         let val: T = *idx;

//         if val.is_zero() {
//             1
//         } else if val.is_one() {
//             self
//         } else {
//             // We pass the new step (val - T::one()) by value, which works
//             // perfectly because a value T also implements Deref<Target = T> via core primitives,
//             // or we can just pass it right into the recursive method.
//             self * self.power(val - T::one())
//         }
//     }
// }

// impl<T: Num + Sub> Power<T, &T> for u32
// where
//     for<'a> &'a T: Sub,
// {
//     type Output = Self;
//     fn power(self, idx: &T) -> <Self as Mul>::Output {
//         if idx.is_zero() {
//             1
//         } else if idx.is_one() {
//             self
//         } else {
//             self * self.power(idx - &T::one())
//         }
//     }
// }

impl Power<u32> for u32 {
    type Output = Self;
    fn power(self, idx: u32) -> <Self as Mul>::Output {
        if idx == 0 {
            1
        } else if idx == 1 {
            self
        } else {
            self * self.power(idx - 1)
        }
    }
}

impl Power<u16> for u32 {
    type Output = Self;
    fn power(self, idx: u16) -> <Self as Mul>::Output {
        if idx == 0 {
            1
        } else if idx == 1 {
            self
        } else {
            self * self.power(idx - 1)
        }
    }
}

impl Power<&u32> for u32 {
    type Output = Self;
    fn power(self, idx: &u32) -> <Self as Mul>::Output {
        if *idx == 0 {
            1
        } else if *idx == 1 {
            self
        } else {
            self * self.power(*idx - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
        assert_eq!(2u32.pow2(3u16), x);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
        assert_eq!(2u32.pow2(3u32), x);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
        assert_eq!(2u32.pow2(&3u32), x);
    }
}
