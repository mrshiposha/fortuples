#[fortuples::auto_impl]
trait Test {
    fn test(box a: Box<i32>) {
        println!("{}", a);
    }
}

fn main() {}
