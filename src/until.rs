use crate::Action;
use crate::Call;
use crate::Unwrapable;

type BorrowedAction<T, U> = for<'a> fn(&'a T) -> U;
// Trait for until
pub trait RepeatableTransform<'a, Prev: Call<&'a From>, From: 'a, Curr, Next> {
    fn until(self, a: Action<Curr, Option<Next>>) -> Until<'a, Prev, From, Curr, Next>;
}
pub struct Until<'a, Prev: Call<&'a From>, From, Curr, Next> {
    to_get_here: Prev,
    go_from_here: fn(&'a From) -> Curr,
    go_if: Action<Curr, Option<Next>>,
}
// TODO!: This does not act like a loop. The result is not passed back into the closure, but instead is discarded.
// This results in the same closure being called with the same arguments in an infinite loop
impl<'a, PrevState: Call<&'a From>, Ret, Curr, From> Call<Ret>
    for Until<'a, PrevState, From, Curr, Ret>
{
    fn call(self) -> Ret {
        let state = self.to_get_here.call();
        let mut i = None;
        loop {
            i = Some((self.go_from_here)(state));
            if let Some(res) = (self.go_if)(i.unwrap()) {
                break res;
            }
        }
    }
}
// Global implementation
impl<'a, PrevState, S, From: 'a, Curr, N> RepeatableTransform<'a, PrevState, From, Curr, N> for S
where
    S: Unwrapable<PrevState, &'a From, Curr>,
    PrevState: Call<&'a From>,
{
    fn until(self, a: Action<Curr, Option<N>>) -> Until<'a, PrevState, From, Curr, N> {
        let (last_state, last_fun) = self.unwrap();
        Until {
            to_get_here: last_state,
            go_from_here: last_fun,
            go_if: a,
        }
    }
}
#[test]
fn test_until() {
    use crate::then::Transform;
    use crate::State;
    let s = State::new(&0)
        .then(|x| {
            println!("{x}");
            x + 1
        })
        .until(|a| (a == 4).then_some(a))
        .dbg();
    panic!("{}", s);
}
