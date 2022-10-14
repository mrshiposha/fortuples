use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::member_type(A)]
    #[tuples::member_type(B)]
    impl Container<#Tuple> {}
}

fn main() {}
