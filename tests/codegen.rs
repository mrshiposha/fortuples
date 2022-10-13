use std::vec::Vec;

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

impl IntoReal for f32 {
    fn into_real(self) -> f32 {
        self
    }
}

struct Vector<Compoenents>(Compoenents);

// tuples::tuple_name / tuples::member_name test
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

// tuples::tuple_name / tuples::member_type test
fortuples! {
    #[tuples::tuple_name(Compoenents)]
    #[tuples::member_type(u32)]
    #[tuples::min_size(2)]
    #[tuples::max_size(3)]
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

trait ToVec {
    type Output;

    fn into_vec(self) -> Vec<Self::Output>;
}

// impl trait generics + tuples::member_type
fortuples! {
    #[tuples::min_size(1)]
    #[tuples::member_type(T)]
    impl<T> ToVec for #Tuple {
        type Output = T;

        fn into_vec(self) -> Vec<Self::Output> {
            vec![#(#self),*]
        }
    }
}

// impl trait generics + lifetimes + tuples::member_type
fortuples! {
    #[tuples::min_size(1)]
    #[tuples::member_type(&'a T)]
    impl<'a, T> Container<#Tuple>
    where
        #(T: IntoReal + Copy),*
    {
        fn lifetimes(&'a self) -> f32 {
            let tuple = &self.0;
            #(#tuple.into_real())+*
        }
    }
}

trait Length {
    fn length(&self) -> usize;
}

// repetition without meta var
fortuples! {
    impl Length for #Tuple {
        fn length(&self) -> usize {
            0#(+1)*
        }
    }
}

trait ConstLength {
    const LENGTH: usize;
}

fortuples! {
    impl ConstLength for #Tuple {
        const LENGTH: usize = #len(Tuple);
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
    assert_eq!(Vector((3, 4.0)).generalized_length() as u32, 5);
    assert_eq!(Vector((1.0, 2.0, 2)).generalized_length() as u32, 3);
}

#[test]
fn test_length() {
    assert_eq!(Vector((5, 12)).length() as u32, 13);
    assert_eq!(Vector((2, 3, 6)).length() as u32, 7);
}

#[test]
fn test_to_vec() {
    assert_eq!((0,).into_vec(), vec![0]);
    assert_eq!((0, 1).into_vec(), vec![0, 1]);
    assert_eq!((0, 1, 2).into_vec(), vec![0, 1, 2]);
    assert_eq!((0, 1, 2, 3).into_vec(), vec![0, 1, 2, 3]);
    assert_eq!((0, 1, 2, 3, 4).into_vec(), vec![0, 1, 2, 3, 4]);
    assert_eq!((0, 1, 2, 3, 4, 5).into_vec(), vec![0, 1, 2, 3, 4, 5]);
    assert_eq!((0, 1, 2, 3, 4, 5, 6).into_vec(), vec![0, 1, 2, 3, 4, 5, 6]);
    assert_eq!(
        (0, 1, 2, 3, 4, 5, 6, 7).into_vec(),
        vec![0, 1, 2, 3, 4, 5, 6, 7]
    );
    assert_eq!(
        (0, 1, 2, 3, 4, 5, 6, 7, 8).into_vec(),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8]
    );
    assert_eq!(
        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9).into_vec(),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    );
    assert_eq!(
        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10).into_vec(),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    );
    assert_eq!(
        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11).into_vec(),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
    );
    assert_eq!(
        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12).into_vec(),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
    );
    assert_eq!(
        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13).into_vec(),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
    );
    assert_eq!(
        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14).into_vec(),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
    );
    assert_eq!(
        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15).into_vec(),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    );
}

#[test]
fn test_lifetimes() {
    assert_eq!(Container((&0i32,)).lifetimes() as u32, 0);
    assert_eq!(Container((&0i32, &1)).lifetimes() as u32, 1);
    assert_eq!(Container((&0i32, &1, &2)).lifetimes() as u32, 3);
    assert_eq!(Container((&0i32, &1, &2, &3)).lifetimes() as u32, 6);
    assert_eq!(Container((&0i32, &1, &2, &3, &4)).lifetimes() as u32, 10);
    assert_eq!(
        Container((&0i32, &1, &2, &3, &4, &5)).lifetimes() as u32,
        15
    );
}

#[test]
fn tuple_length() {
    assert_eq!(().length(), 0);
    assert_eq!((0,).length(), 1);
    assert_eq!((0, 'a').length(), 2);
    assert_eq!((0, 'a', 3.14).length(), 3);
}

#[test]
fn test_const_length() {
    assert_eq!(<() as ConstLength>::LENGTH, ().length());
    assert_eq!(<(u32,) as ConstLength>::LENGTH, (0,).length());
    assert_eq!(<(u32, char) as ConstLength>::LENGTH, (0, 'a').length());
    assert_eq!(<(u32, char, f32) as ConstLength>::LENGTH, (0, 'a', 3.14).length());
}
