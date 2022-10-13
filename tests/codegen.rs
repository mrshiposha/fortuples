use fortuples::fortuples;

struct Container<T>(T);

fortuples! {
    #[tuples::min_size(1)]
    #[tuples::max_size(5)]
    impl Container<#Tuple>
    where
        #(#Member: Into<i32>),*
    {
        fn sum(self) -> i32 {
            let tuple = self.0;
            #(#tuple.into())+*
        }
    }
}

trait IntoReal {
    fn into_real(self) -> f32;
}

impl IntoReal for i32 {
    fn into_real(self) -> f32 {
        self as f32
    }
}

struct Vector<Compoenents>(Compoenents);

fortuples! {
    #[tuples::tuple_name(Compoenents)]
    #[tuples::member_name(Component)]
    #[tuples::min_size(2)]
    #[tuples::max_size(3)]
    impl Vector<#Compoenents>
    where
        #(#Component: IntoReal + Copy),*
    {
        fn generalized_length(&self) -> f32 {
            let coords = &self.0;
            let squares = #({
                let f_coords = #coords.into_real();
                f_coords * f_coords
            })+*;

            squares.sqrt()
        }
    }
}

fortuples! {
    #[tuples::tuple_name(Compoenents)]
    #[tuples::member_type(u32)]
    #[tuples::min_size(2)]
    #[tuples::max_size(3)]
    #[tuples::debug_expand(path = "expand.rs")]
    impl Vector<#Compoenents> {
        fn length(&self) -> f32 {
            let coords = &self.0;
            let squares = #({
                let f_coords = #coords as u32;
                f_coords * f_coords
            })+*;

            (squares as f32).sqrt()
        }
    }

}

#[test]
fn test_sum() {
    assert_eq!(Container((1,)).sum(), 1);
    assert_eq!(Container((1, 2)).sum(), 3);
    assert_eq!(Container((1, 2, 3)).sum(), 6);
    assert_eq!(Container((1, 2, 3, 4)).sum(), 10);
    assert_eq!(Container((1, 2, 3, 4, 5)).sum(), 15);
}

#[test]
fn test_generalized_length() {
    assert_eq!(Vector((3, 4)).generalized_length() as u32, 5);
    assert_eq!(Vector((1, 2, 2)).generalized_length() as u32, 3);
}

#[test]
fn test_length() {
    assert_eq!(Vector((5, 12)).generalized_length() as u32, 13);
    assert_eq!(Vector((2, 3, 6)).generalized_length() as u32, 7);
}
