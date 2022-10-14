use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::debug_expand(path = "not/existing/file")]
    impl Container<#Tuple> {}
}

fn main() {}
