use std::marker::PhantomData;

trait CallTwice<'b, A, T> {
    fn call_twice(self, t: &'b A);
}
struct Store<A, B, T> {
    v: T,
    _boo: PhantomData<(A, B)>,
}
fn new<A, B>(t: fn(A) -> B) -> Store<A, B, fn(A) -> B> {
    Store {
        v: t,
        _boo: PhantomData,
    }
}
impl<'a, 'b: 'a, B, F> CallTwice<'b, B, F> for Store<&'a B, F, fn(&'b B) -> F> {
    fn call_twice(self, t: &'b B) {
        let fun = self.v;
        (fun)(t);
        (fun)(t);
    }
}
impl<A, B> Store<A, B, fn(A) -> B> {
    fn call(self, t: A) {
        (self.v)(t);
    }
}
