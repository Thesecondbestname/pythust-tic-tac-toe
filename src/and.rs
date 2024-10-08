use super::{Action, Call, PhantomData};
use crate::Unwrapable;
pub trait UnwrapTransform<LastState, State, Prev, Curr>
where
    LastState: Call<Prev>,
    State: Unwrapable<LastState, Prev, Curr> + Call<Curr>,
{
    fn and<Next>(self, f: fn(Curr) -> Next) -> And<LastState, State, Prev, Curr, Next>;
}

struct And<
    LastState: Call<Prev>,
    State: Call<Curr> + Unwrapable<LastState, Prev, Curr>,
    Prev,
    Curr,
    Next,
> {
    to_get_there: State,
    curr_fun: Action<Curr, Next>,
    p: PhantomData<(LastState, Prev, Curr)>,
}
impl<LastState, Prev, Curr, State, Next> Call<Next> for And<LastState, State, Prev, Curr, Next>
where
    LastState: Call<Prev>,
    State: Call<Curr> + Unwrapable<LastState, Prev, Curr>,
{
    #[inline(always)]
    fn call(self) -> Next {
        let (last_state, prev_fun) = self.to_get_there.unwrap();
        (self.curr_fun)(prev_fun(last_state.call()))
    }
}
impl<To, State, Curr, LastState, Prev> Unwrapable<State, Curr, To>
    for And<LastState, State, Prev, Curr, To>
where
    State: Call<Curr> + Unwrapable<LastState, Prev, Curr>,
    LastState: Call<Prev>,
{
    fn unwrap(self) -> (State, Action<Curr, To>) {
        (self.to_get_there, self.curr_fun)
    }
}
impl<PrevS, S, PrevT, T> UnwrapTransform<PrevS, Self, PrevT, T> for S
where
    PrevS: Call<PrevT>,
    S: Call<T> + Unwrapable<PrevS, PrevT, T>,
{
    #[inline]
    fn and<Next>(self, f: fn(T) -> Next) -> And<PrevS, S, PrevT, T, Next> {
        And {
            to_get_there: self,
            curr_fun: f,
            p: PhantomData,
        }
    }
}
