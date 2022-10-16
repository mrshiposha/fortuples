use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::refs_tuple]
    #[tuples::refs_tuple(mut)]
    impl Container<#Tuple> {}
}

fn main() {}
