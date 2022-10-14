use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::debug_expand(invalid = "")]
    impl Container<#Tuple> {}
}

fn main() {}
