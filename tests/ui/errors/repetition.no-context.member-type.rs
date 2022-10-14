use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    impl Container<#Tuple> {
        #Member
    }
}

fn main() {}
