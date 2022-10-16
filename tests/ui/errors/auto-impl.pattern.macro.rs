#[fortuples::auto_impl]
trait Test {
    fn test(some_macro!(a): i32) {
        println!("{}", a);
    }
}

fn main() {}
