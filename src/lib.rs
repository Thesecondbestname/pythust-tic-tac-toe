#![allow(dead_code)]

use std::marker::PhantomData;
type TransformAction<T, U> = Box<(dyn Fn(T) -> U + 'static)>;
type Action<T> = Box<(dyn Fn(T) -> T)>;

trait Call<Ret> {
    fn call(self) -> Ret;
}
// struct Act<T, U>(T, Box<(dyn Fn(T) -> U)>);
// impl<T, U> Call<U> for Act<T,U> {
//     fn call(self) -> U {
//         (self.1)(self.0)
//     }
// }
struct RecState<From, Fn: Call<From>, To> {
    to_get_there: Fn,
    go_from_here: TransformAction<From, To>,
}
struct State2<T> {
    initial: T
}
struct InitialState<From, To> {
    state: From,
    go_from_here: TransformAction<From, To>
}

impl<From, To> InitialState<From, To> where InitialState<From, To>: Call<To>, From: 'static, To: 'static {
    fn then<Next: 'static>(self, f: fn(To) -> Next) -> RecState<To, InitialState<From, To>, Next> {
        RecState {
            to_get_there: self,
            go_from_here: Box::new(f),
        }
    }
}
impl<T, U: Call<T>, V> Call<V> for RecState<T, U, V> {
    fn call(self) -> V {
        (self.go_from_here)(self.to_get_there.call())
    }
}
impl<From,To> Call<To> for InitialState<From, To> {
    fn call(self) -> To {
        (self.go_from_here)(self.state)
    }
}
impl<To> Call<To> for State2<To> {
    fn call(self) -> To {
        self.initial
    }
}
impl<From: 'static> State2<From> {
    fn new(initial: From) -> Self {
        State2 {
            initial
        }   
    }
    fn then<To: 'static>(self, f: fn(From) -> To) -> InitialState<From, To> {
        InitialState{
            state: self.initial,
            go_from_here: Box::new(f),
        }
    }
}
impl<Curr, T, To>  RecState<Curr, T, To> where
     RecState<Curr, T, To>: Call<Curr>,
     Curr: 'static,
     T: Call<Curr> 
{
    fn then<Next: 'static>(self, f: fn(Curr) -> Next) -> RecState<Curr, RecState<Curr, T, To>, Next> {
        RecState {
            to_get_there: self,
            go_from_here: Box::new(f),
        }
    }
    fn dbg(self) -> Curr {
        self.call()
    }
}
trait TransformType<Curr, T: Call<Curr>> {
    fn dbg(self) -> Curr;
    fn then<Next: 'static>(self, f: fn(Curr) -> Next) -> RecState<Curr, T, Next>;
    // fn until<V: 'static>(self, transformation: fn(&I) -> Option<V>) -> Until<I,Target>;
}

impl<Curr, T> TransformType<Curr, T> for T where Curr: 'static, T: Call<Curr> {
    fn dbg(self) -> Curr {
        self.call()
    }
    fn then<Next: 'static>(self, f: fn(Curr) -> Next) -> RecState<Curr, T, Next> {
        RecState {
            to_get_there: self,
            go_from_here: Box::new(f),
        }
    }
}

struct State<T> where T: 'static + Default {
    state: T,
    actions: Vec<Action<T>>,
}
trait Transformation<T> {
    type Output;
    fn dbg(self) -> T;
    fn run(self);
    fn then(self, f: fn(T) -> T) -> Self::Output;
    fn and(self, f: fn(T) -> T) -> Self::Output;
}

impl<T: Default + std::fmt::Debug> Transformation<T> for State<T> {
    type Output = State<T>;
    fn dbg(self) -> T {
        let mut state = self.state;
        let len = self.actions.len();
        for f in self.actions {
            state = f(state);
            dbg!(&state, len);
        }
        state
    }
    fn then(self, f: fn(T) -> T) -> Self::Output {
        let mut a = self.actions;
        a.push(Box::new(f));
        Self {
            state: self.state,
            actions: a,
        }
    }
    fn and(self, f: fn(T) -> T) -> Self::Output {
        let mut actions = self.actions;
        let last_act = actions.pop().expect("Should be initialized with at least one in the actions array");
        actions.push(Box::new(move |state| f(last_act(state))));
        Self {
            actions,
            ..self
        }
    }
    fn run(self) {
        let mut state = self.state;
        for f in self.actions {
            state = f(state)
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_then(){
        assert_eq!(
            State2::new(
                "Call 1"
            ).then(|arg|{println!("{arg}"); 6}).then(|arg| {println!("{arg}"); "Call 3"}).dbg(),
         "Call 3")
    }
    // #[test]
    // fn test_and() {
    //     assert_eq!(State2::new(
    //         "Call 1"
    //     ).then(|_| {println!("yees"); "Call 2"}).dbg(), "Call 2")
    // }
    fn play_around() {
    }
}
