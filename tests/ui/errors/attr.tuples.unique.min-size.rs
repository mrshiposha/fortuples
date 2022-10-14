use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::min_size(1)]
    #[tuples::min_size(2)]
    impl Container<#Tuple> {}
}

fn main() {}
