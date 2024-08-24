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
trait Previous<State: Call<Prev>, Prev, Curr> {
    fn prev(self) -> (State, impl Fn(Prev) -> Curr);
}
trait Transform<Curr, T: Call<Curr>>: Call<Curr> {
    fn dbg(self) -> Curr;
    fn then<Next: 'static>(self, f: fn(Curr) -> Next) -> RecState<Curr, T, Next>;
}
trait PrevAwareTransform<
    LastState: Call<Prev>,
    State: Previous<LastState, Prev, Curr> + Call<Curr>,
    Prev,
    Curr,
>
{
    fn and<Next>(self, f: fn(Curr) -> Next) -> And<LastState, State, Prev, Curr, Next>;
}
struct RecState<Curr, State: Call<Curr>, Next> {
    to_get_there: State,
    go_from_here: Action<Curr, Next>,
}
// struct And<State, Prev, Curr, Next, F: (Fn(Prev) -> Next)> {
//     to_get_there: State,
//     curr_fn: F,
//     f: PhantomData<(Prev, Curr, Next)>,
// }
struct And<
    LastState: Call<Prev>,
    State: Call<Curr> + Previous<LastState, Prev, Curr>,
    Prev,
    Curr,
    Next,
> {
    to_get_there: State,
    go_from_here: Action<Curr, Next>,
    p: PhantomData<(LastState, Prev, Curr)>,
}
impl<LastState, Prev, Curr, State, Next> Call<Next> for And<LastState, State, Prev, Curr, Next>
where
    LastState: Call<Prev>,
    State: Call<Curr> + Previous<LastState, Prev, Curr>,
{
    fn call(self) -> Next {
        let (last_state, prev_fun) = self.to_get_there.prev();
        (self.go_from_here)(prev_fun(last_state.call()))
    }
}
impl<To, State, Curr, LastState, Prev> Previous<State, Curr, To>
    for And<LastState, State, Prev, Curr, To>
where
    State: Call<Curr> + Previous<LastState, Prev, Curr>,
    LastState: Call<Prev>,
{
    fn prev(self) -> (State, impl (Fn(Curr) -> To)) {
        (self.to_get_there, self.go_from_here)
    }
}
impl<T, U: Call<T>, V> Call<V> for RecState<T, U, V> {
    fn call(self) -> V {
        (self.go_from_here)(self.to_get_there.call())
    }
}
impl<To> Call<To> for State<To> {
    fn call(self) -> To {
        self.initial
    }
}
// impl<Prev, Curr, Last, Next, F> Call<Next> for And<Last, Prev, Curr, Next, F>
// where
//     Last: Call<Prev>,
//     F: (Fn(Prev) -> Next),
// {
//     fn call(self) -> Next {
//         let ret = self.to_get_there.call();
//         (self.curr_fn)(ret)
//     }
// }
impl<To, State: Call<Curr>, Curr> Previous<State, Curr, To> for RecState<Curr, State, To> {
    fn prev(self) -> (State, impl Fn(Curr) -> To) {
        (self.to_get_there, (self.go_from_here))
    }
}
impl<LastState, State, Prev, Curr> PrevAwareTransform<LastState, Self, Prev, Curr> for State
where
    LastState: Call<Prev>,
    State: Call<Curr> + Previous<LastState, Prev, Curr>,
{
    fn and<Next>(self, f: fn(Curr) -> Next) -> And<LastState, State, Prev, Curr, Next> {
        And {
            to_get_there: self,
            go_from_here: f,
            p: PhantomData,
        }
    }
}
impl<Curr, T> Transform<Curr, T> for T
where
    Curr: 'static,
    T: Call<Curr>,
{
    fn dbg(self) -> Curr {
        self.call()
    }
    fn then<Next: 'static>(self, f: fn(Curr) -> Next) -> RecState<Curr, T, Next> {
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
