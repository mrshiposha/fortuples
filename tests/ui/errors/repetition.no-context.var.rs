use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    impl Container<#Tuple> {
        #myvar
    }
}

fn main() {}
