use crate::Action;
use crate::Call;

type BorrowedAction<T, U> = for<'a> fn(&'a T) -> U;
trait Repeatable<State: Call<Prev>, Prev, Curr> {
    fn unwrap(self) -> (State, BorrowedAction<Prev, Curr>);
}
pub trait RepeatableTransform<Prev: Call<From>, From, Curr, Next> {
    fn until(self, a: Action<Curr, Option<Next>>) -> Until<Prev, From, Curr, Next>;
}
pub trait BorrowedTransform<Curr>: Call<Curr> + Sized {
    fn dbg(self) -> Curr;
    fn then<Next: 'static>(self, f: fn(&Curr) -> Next) -> BorrowedRecState<Self, Curr, Next>;
}
struct BorrowedRecState<State: Call<Curr>, Curr, Next> {
    to_get_there: State,
    go_from_here: BorrowedAction<Curr, Next>,
}
struct Until<Prev: Call<From>, From, Curr, Next> {
    to_get_here: Prev,
    go_from_here: BorrowedAction<From, Curr>,
    go_if: Action<Curr, Option<Next>>,
}
impl<T, U: Call<T>, V> Call<V> for BorrowedRecState<U, T, V> {
    fn call(self) -> V {
        (self.go_from_here)(&self.to_get_there.call())
    }
}
impl<State: Call<Curr>, Curr, Next> Repeatable<State, Curr, Next>
    for BorrowedRecState<State, Curr, Next>
{
    fn unwrap(self) -> (State, BorrowedAction<Curr, Next>) {
        (self.to_get_there, self.go_from_here)
    }
}
impl<PrevState: Call<From>, Ret, Curr, From> Call<Ret> for Until<PrevState, From, Curr, Ret> {
    fn call(self) -> Ret {
        let state = self.to_get_here.call();
        loop {
            let i = (self.go_from_here)(&state);
            if let Some(res) = (self.go_if)(i) {
                break res;
            }
        }
    }
}
impl<PrevState, S, From, Curr, N> RepeatableTransform<PrevState, From, Curr, N> for S
where
    S: Repeatable<PrevState, From, Curr>,
    PrevState: Call<From>,
{
    fn until(self, a: Action<Curr, Option<N>>) -> Until<PrevState, From, Curr, N> {
        let (last_state, last_fun) = self.unwrap();
        Until {
            to_get_here: last_state,
            go_from_here: last_fun,
            go_if: a,
        }
    }
}
impl<Curr, S: Call<Curr>> BorrowedTransform<Curr> for S {
    fn dbg(self) -> Curr {
        self.call()
    }

    fn then<Next: 'static>(self, f: fn(&Curr) -> Next) -> BorrowedRecState<Self, Curr, Next> {
        BorrowedRecState {
            to_get_there: self,
            go_from_here: f,
        }
    }
}

#[test]
fn test_until() {
    use crate::State;
    let s = State::new(0).then(|x| x + 40).and(|a| {
        println!("{a}");
        a
    });
}
