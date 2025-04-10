use crate::Unwrapable;
use std::marker::PhantomData;

use crate::Call;

// trait for then
pub trait Transform<Curr>: Call<Curr> + Sized {
    fn dbg(self) -> Curr;
    fn then<Next>(self, f: fn(Curr) -> Next) -> RecState<fn(Curr) -> Next, Self, Curr, Next>;
}
pub struct RecState<F, State: Call<Curr>, Curr, Next> {
    to_get_there: State,
    go_from_here: F,
    _boo: PhantomData<(Next, Curr)>,
}
impl<T, U: Call<T>, V> Call<V> for RecState<fn(T) -> V, U, T, V> {
    fn call(self) -> V {
        (self.go_from_here)(self.to_get_there.call())
    }
}
impl<Curr, State: Call<Prev>, Prev> Unwrapable<State, Prev, Curr>
    for RecState<fn(Prev) -> Curr, State, Prev, Curr>
{
    fn unwrap(self) -> (State, fn(Prev) -> Curr) {
        (self.to_get_there, (self.go_from_here))
    }
}
// Global implementation
impl<C, T> Transform<C> for T
where
    T: Call<C>,
{
    #[inline]
    fn dbg(self) -> C {
        self.call()
    }
    fn then<N>(self, f: fn(C) -> N) -> RecState<fn(C) -> N, Self, C, N> {
        RecState {
            to_get_there: self,
            go_from_here: f,
            _boo: PhantomData,
        }
    }
}
#[test]
fn test_then() {
    use crate::State;
    let _x = State::new(0).then(|a| a + 43);
    let _x = State::new(&0).then(|a: &i32| a + 43).call();
}
