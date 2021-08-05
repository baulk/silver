use std::env;
// https://docs.rs/git2/0.13.20/git2/
//
fn main() {
    for argument in env::args() {
        println!("{}", argument);
    }
   // let e=Executor::create()
}
