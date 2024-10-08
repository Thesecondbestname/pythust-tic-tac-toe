#![allow(dead_code)]
mod and;
mod then;
mod until;

use std::marker::PhantomData;

struct State<T> {
    initial: T,
}
impl<From: 'static> State<From> {
    const fn new(initial: From) -> Self {
        Self { initial }
    }
}

type Action<T, U> = fn(T) -> U;
trait Call<Ret> {
    fn call(self) -> Ret;
}

/// Provides a tuple of (State, Action<Prev, Curr>)
trait Unwrapable<State: Call<Prev>, Prev, Curr> {
    fn unwrap(self) -> (State, Action<Prev, Curr>);
}

// Call impls
impl<T> Call<T> for State<T> {
    fn call(self) -> T {
        self.initial
    }
}

#[cfg(test)]
mod tests {

    use crate::{and::UnwrapTransform, then::Transform};

    use super::*;
    #[test]
    fn test_then() {
        assert_eq!(
            State::new("Call 1")
                .then(|arg| {
                    println!("{arg}");
                    6
                })
                .then(|arg| {
                    println!("{arg}");
                    "Call 3"
                })
                .dbg(),
            "Call 3"
        );
    }
    fn add2(a: u8) -> u8 {
        a + 2
    }
    #[test]
    fn test_and() {
        assert_eq!(
            State::new(0)
                .then(add2)
                .and(|_| {
                    print!("Geht's?");
                    0
                })
                .and(|i| {
                    println!("Fuck yeah {}", i);
                    "test"
                })
                .then(str::len)
                .and(|_| 0)
                .dbg(),
            0
        );
    }
    fn play_around() {
        State::new(3)
            .then(|a| a + 5)
            .until(|a| (a > 5).then_some(a))
    }
}
