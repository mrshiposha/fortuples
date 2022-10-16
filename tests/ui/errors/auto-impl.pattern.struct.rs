#[fortuples::auto_impl]
trait Test {
    fn test(Foo { a }: Foo) {
        println!("{}", a);
    }
}

fn main() {}
