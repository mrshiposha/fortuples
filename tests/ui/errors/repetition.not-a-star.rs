use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    impl Container<#Tuple> {} #(),$
}

fn main() {}
