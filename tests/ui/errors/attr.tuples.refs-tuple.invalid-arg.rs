use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::refs_tuple(A)]
    impl Container<#Tuple> {}
}

fn main() {}
