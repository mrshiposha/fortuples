use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::max_size(1)]
    #[tuples::max_size(2)]
    impl Container<#Tuple> {}
}

fn main() {}
