use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::member_name(A)]
    #[tuples::member_name(B)]
    impl Container<#Tuple> {}
}

fn main() {}
