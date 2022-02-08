#![cfg_attr(not(test), no_std)]
///! Embedded time wrapper for RTIC's `monotonics::now()`
///!
///! # Design
///!  `Clock` is implemented using the RTIC `app::monotonics::now()` which is backed
///!   by a monotonic `Monotonic`.
use core::hash::Hash;
pub use embedded_time;
use embedded_time::{clock::Error, fraction::Fraction, Clock, Instant, TimeInt};

#[derive(Copy, Clone, Debug)]
pub struct MonoClock<T, const HZ: u32>(fn() -> T);

impl<T, const HZ: u32> MonoClock<T, HZ> {
    ///! Create a new `MonoClock` using e.g. RTIC's `monotonics::now()`.
    ///!
    ///! Args:
    ///! * now: a closure that returns the current ticks
    ///!
    ///! ```
    ///! // In your `app` `init()`, set up a `Monotonic` as usual, e.g.:
    ///! use mono_clock::MonoClock, systick_monotonic::Systick;
    ///! const HZ: u32 = 1_000;
    ///! let sysclk = 400_000_000u32;
    ///! let mono = Systick::<HZ>::new(c.core.SYST, sysclk);
    ///! // Then build a `Clock` that is `Copy` and can be passed
    ///! // around by value or reference:
    ///! let clock = MonoClock::<u32, HZ>::new(|| monotonics::now());
    ///! ```
    pub fn new(now: fn() -> T) -> Self {
        Self(now)
    }
}

impl<T: Copy + TimeInt + Hash, const HZ: u32> Clock for MonoClock<T, HZ> {
    type T = T;

    // The duration of each tick in seconds.
    const SCALING_FACTOR: Fraction = Fraction::new(1, HZ);

    fn try_now(&self) -> Result<Instant<Self>, Error> {
        Ok(Instant::new((self.0)() as T))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let c = MonoClock::<u32, 1_000>::new(|| 42);
        assert_eq!(c.try_now(), Ok(Instant::<MonoClock<_, 1_000>>::new(42)));
    }
}
