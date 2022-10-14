use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::tuple_name(A)]
    #[tuples::tuple_name(B)]
    impl Container<#Tuple> {}
}

fn main() {}
