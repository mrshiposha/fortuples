#[fortuples::auto_impl]
trait Test {
    fn test(&a: &i32) {
        println!("{}", a);
    }
}

fn main() {}
