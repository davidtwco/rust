//@ run-rustfix

struct S;
trait Trait {
    fn foo() {}
}
impl Trait for &mut S {}
trait Trait2 {
    fn bar() {}
}
impl Trait2 for &S {}
impl Trait2 for &mut S {}
fn main() {
    let _ = &str::from("value");
    //~^ ERROR the trait bound `str: From<_>` is not satisfied
    //~| ERROR the size for values of type `str` cannot be known at compilation time
    let _ = &mut S::foo();
    //~^ ERROR the trait bound `S: Trait` is not satisfied
    let _ = &S::foo();
    //~^ ERROR the trait bound `S: Trait` is not satisfied
    let _ = S::foo();
    //~^ ERROR the trait bound `S: Trait` is not satisfied
    let _ = &mut S::bar();
    //~^ ERROR the trait bound `S: Trait2` is not satisfied
    let _ = &S::bar();
    //~^ ERROR the trait bound `S: Trait2` is not satisfied
}
