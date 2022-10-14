use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::debug_expand()]
    impl Container<#Tuple> {}
}

fn main() {}
