use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::too::many]
    impl Container<#Tuple> {}
}

fn main() {}
