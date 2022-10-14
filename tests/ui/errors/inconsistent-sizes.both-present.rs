use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::min_size(2)]
    #[tuples::max_size(1)]
    impl Container<#Tuple> {}
}

fn main() {}
