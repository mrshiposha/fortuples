use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::max_size(0)]
    impl Container<#Tuple> {}
}

fn main() {}
