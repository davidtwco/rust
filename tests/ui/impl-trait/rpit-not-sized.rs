fn foo() -> impl ?Sized {
    //~^ ERROR the size for values of type `impl MetaSized` cannot be known at compilation time
    ()
}

fn main() {}
