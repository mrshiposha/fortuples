use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::debug_expand]
    #[tuples::debug_expand(path = "")]
    impl Container<#Tuple> {}
}

fn main() {}
