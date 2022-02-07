///! Embedded time wrapper for RTIC's `monotonics::now()`
///!
///! # Design
///!  `Clock` is implemented using the RTIC `app::monotonics::now()` which is backed
///!   by a monotonic `Monotonic`.
use core::hash::Hash;
pub use embedded_time::Clock;
use embedded_time::{self as et, fraction::Fraction};

#[derive(Copy, Clone, Debug)]
pub struct MonoClock<T: Copy, const HZ: u32>(fn() -> T);

impl<T: Copy, const HZ: u32> MonoClock<T, HZ> {
    ///! Create a new `MonoClock` using e.g. RTIC's `monotonics::now()`.
    ///!
    ///! Args:
    ///! * now: a closure that returns the current ticks
    ///!
    ///! ```
    ///! use mono_clock::MonoClock;
    ///! let clock = MonoClock::new(|| monotonics::now());
    ///! ```
    pub fn new(now: fn() -> T) -> Self {
        Self(now)
    }
}

impl<T: Copy + et::TimeInt + Hash, const HZ: u32> et::Clock for MonoClock<T, HZ> {
    type T = T;

    // The duration of each tick in seconds.
    const SCALING_FACTOR: Fraction = Fraction::new(1, HZ);

    fn try_now(&self) -> Result<et::Instant<Self>, et::clock::Error> {
        Ok(et::Instant::new((self.0)() as T))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let c = MonoClock::<u32, 1_000>::new(|| 42);
        assert_eq!(c.try_now(), Ok(et::Instant::<MonoClock<_, 1_000>>::new(42)));
    }
}
