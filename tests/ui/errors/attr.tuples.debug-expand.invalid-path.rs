use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::debug_expand(path = 2)]
    impl Container<#Tuple> {}
}

fn main() {}
