#[fortuples::auto_impl]
trait Test {
    fn test((a, b): (i32, char)) {
        println!("({}, {})", a, b);
    }
}

fn main() {}
