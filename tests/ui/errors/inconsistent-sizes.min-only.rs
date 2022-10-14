use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::min_size(16)]
    impl Container<#Tuple> {}
}

fn main() {}
