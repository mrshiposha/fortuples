use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::unknown]
    impl Container<#Tuple> {}
}

fn main() {}
