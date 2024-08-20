#![allow(dead_code)]

type Action<T, U> = fn(T) -> U;
trait Call<Ret> {
    fn call(self) -> Ret;
}
trait Transform<Curr, T: Call<Curr>> : Call<Curr> {
    fn dbg(self) -> Curr;
    fn then<Next: 'static>(self, f: fn(Curr) -> Next) -> RecState<Curr, T, Next>;
}
struct RecState<From, Fn: Call<From>, To> {
    to_get_there: Fn,
    go_from_here: Action<From, To>,
}
struct State<T> {
    initial: T
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
impl<From: 'static> State<From> {
    fn new(initial: From) -> Self {
        State {
            initial
        }   
    }
}
impl<Curr, T> Transform<Curr, T> for T where Curr: 'static, T: Call<Curr> {
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

mod out_of_sight_out_of_mind {
    type TransformAction<T, U> = fn(T) -> U;
    trait Call<Ret> {
        fn call(self) -> Ret;
    }
    trait TransformType<Curr, T: Call<Curr>> : Call<Curr> {
        fn dbg(self) -> Curr;
        fn then<Next: 'static, Fun: (Fn(Curr) -> Next)>(self, f: fn(Curr) -> Next) -> RecState<Curr, Fun, Next>;
        // fn and<Next: 'static>(self, f: fn(Curr) -> Next) -> RecState<Curr, T, Next>;
        // fn until<V: 'static>(self, transformation: fn(&I) -> Option<V>) -> Until<I,Target>;
    }
    struct RecState<From, Fun: (Fn(From) -> To), To> {
        to_get_there: Fun,
        go_from_here: TransformAction<From, To>,
    }
    struct State<T> {
        initial: T
    }
    impl<Curr, T, To>  RecState<Curr, T, To> where
         RecState<Curr, T, To>: Call<Curr>,
         Curr                 : 'static,
         T                    : Call<Curr> + (Fn(Curr) -> To)
    {
    }
    impl<T, U: Call<T> + (Fn(T) -> V), V> Call<V> for RecState<T, U, V> {
        fn call(self) -> V {
            (self.go_from_here)(self.to_get_there.call())
        }
    }
    impl<To> Call<To> for State<To> {
        fn call(self) -> To {
            self.initial
        }
    }
    impl<From: 'static> State<From> {
        fn new(initial: From) -> Self {
            State {
                initial
            }   
        }
    }

    trait Transformation<T> {
        type Output;
        fn dbg(self) -> T;
        fn run(self);
        fn then(self, f: fn(T) -> T) -> Self::Output;
        fn and(self, f: fn(T) -> T) -> Self::Output;
    }
    struct Until<From, Fn: Call<From>, To> {
        to_get_there: Fn,
        go_from_here: TransformAction<From, To>,
        go_if: TransformAction<From, To>,
    }
    
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_then(){
        assert_eq!(
            State::new(
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
