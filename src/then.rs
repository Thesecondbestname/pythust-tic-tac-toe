use std::marker::PhantomData;

use crate::Action;
use crate::Call;

/// Provides a tuple of (State, Action<Prev, Curr>)
trait Unwrapable<State: Call<Prev>, Prev, Curr> {
    fn unwrap(self) -> (State, impl FnOnce(Prev) -> Curr);
}
pub trait Transform<Curr>: Call<Curr> + Sized {
    fn dbg(self) -> Curr;
    fn then<Next, F: (FnOnce(Curr) -> Next)>(self, f: F) -> RecState<F, Self, Curr, Next>;
}
struct RecState<F, State: Call<Curr>, Curr, Next> {
    to_get_there: State,
    go_from_here: F,
    _boo: PhantomData<(Next, Curr)>,
}
impl<F: FnOnce(T) -> V, T, U: Call<T>, V> Call<V> for RecState<F, U, T, V> {
    fn call(self) -> V {
        (self.go_from_here)(self.to_get_there.call())
    }
}
impl<F: Fn(Prev) -> Curr, Curr, State: Call<Prev>, Prev> Unwrapable<State, Prev, Curr>
    for RecState<F, State, Prev, Curr>
{
    fn unwrap(self) -> (State, impl FnOnce(Prev) -> Curr) {
        (self.to_get_there, (self.go_from_here))
    }
}
impl<C, T> Transform<C> for T
where
    T: Call<C>,
{
    #[inline]
    fn dbg(self) -> C {
        self.call()
    }
    fn then<N, F: (FnOnce(C) -> N)>(self, f: F) -> RecState<F, T, C, N> {
        RecState {
            to_get_there: self,
            go_from_here: f,
            _boo: PhantomData,
        }
    }
}
// #[test]
// fn test_then() {
//     use crate::until::BorrowedTransform;
//     use crate::State;
//     let x = State::new(0).then(|a| a + 43);
// }
