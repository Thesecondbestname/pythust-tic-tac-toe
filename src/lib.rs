#![allow(dead_code)]

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
type BorrowedAction<T, U> = for<'a> fn(&'a T) -> U;
trait Call<Ret> {
    fn call(self) -> Ret;
}
trait Unwrapable<State: Call<Prev>, Prev, Curr> {
    fn unwrap(self) -> (State, Action<Prev, Curr>);
}
trait Transform<Curr, T: Call<Curr>>: Call<Curr> {
    fn dbg(self) -> Curr;
    fn then<Next: 'static>(self, f: fn(Curr) -> Next) -> RecState<T, Curr, Next>;
}
trait UnwrapTransform<LastState, State, Prev, Curr>
where
    LastState: Call<Prev>,
    State: Unwrapable<LastState, Prev, Curr> + Call<Curr>,
{
    fn and<Next>(self, f: fn(Curr) -> Next) -> And<LastState, State, Prev, Curr, Next>;
}
// Structs
struct RecState<State: Call<Curr>, Curr, Next> {
    to_get_there: State,
    go_from_here: Action<Curr, Next>,
}
struct And<
    LastState: Call<Prev>,
    State: Call<Curr> + Unwrapable<LastState, Prev, Curr>,
    Prev,
    Curr,
    Next,
> {
    to_get_there: State,
    go_from_here: Action<Curr, Next>,
    p: PhantomData<(LastState, Prev, Curr)>,
}
// Call impls
impl<LastState, Prev, Curr, State, Next> Call<Next> for And<LastState, State, Prev, Curr, Next>
where
    LastState: Call<Prev>,
    State: Call<Curr> + Unwrapable<LastState, Prev, Curr>,
{
    #[inline(always)]
    fn call(self) -> Next {
        let (last_state, prev_fun) = self.to_get_there.unwrap();
        (self.go_from_here)(prev_fun(last_state.call()))
    }
}
impl<T, U: Call<T>, V> Call<V> for RecState<U, T, V> {
    fn call(self) -> V {
        (self.go_from_here)(self.to_get_there.call())
    }
}
impl<T> Call<T> for State<T> {
    fn call(self) -> T {
        self.initial
    }
}
// Unwrap impls
impl<Curr, State: Call<Prev>, Prev> Unwrapable<State, Prev, Curr> for RecState<State, Prev, Curr> {
    fn unwrap(self) -> (State, Action<Prev, Curr>) {
        (self.to_get_there, (self.go_from_here))
    }
}
impl<To, State, Curr, LastState, Prev> Unwrapable<State, Curr, To>
    for And<LastState, State, Prev, Curr, To>
where
    State: Call<Curr> + Unwrapable<LastState, Prev, Curr>,
    LastState: Call<Prev>,
{
    fn unwrap(self) -> (State, Action<Curr, To>) {
        (self.to_get_there, self.go_from_here)
    }
}
impl<PrevS, S, PrevT, T> UnwrapTransform<PrevS, Self, PrevT, T> for S
where
    PrevS: Call<PrevT>,
    S: Call<T> + Unwrapable<PrevS, PrevT, T>,
{
    #[inline(always)]
    fn and<Next>(self, f: fn(T) -> Next) -> And<PrevS, S, PrevT, T, Next> {
        And {
            to_get_there: self,
            go_from_here: f,
            p: PhantomData,
        }
    }
}
impl<C, T> Transform<C, T> for T
where
    C: 'static,
    T: Call<C>,
{
    #[inline(always)]
    fn dbg(self) -> C {
        self.call()
    }
    fn then<N>(self, f: fn(C) -> N) -> RecState<T, C, N> {
        RecState {
            to_get_there: self,
            go_from_here: f,
        }
    }
}
mod later {
    use super::Action;
    use super::Call;
    trait Transformation<T> {
        type Output;
        fn dbg(self) -> T;
        fn run(self);
        fn then(self, f: fn(T) -> T) -> Self::Output;
        fn and(self, f: fn(T) -> T) -> Self::Output;
    }
    struct Until<From, Fn: Call<From>, To> {
        to_get_there: Fn,
        go_from_here: Action<From, To>,
        go_if: Action<From, To>,
    }
}

#[cfg(test)]
mod tests {

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
                .and(|i| {
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
    fn play_around() {}
}
