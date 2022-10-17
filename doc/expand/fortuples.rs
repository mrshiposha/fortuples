impl<Member0> Trait for (Member0,)
where
    Member0: Trait<FixedType = i32>,
{
    type Ret = (Member0::Ret,);
    type Arg = (Member0::Arg,);
    const VALUE: i32 = Member0::VALUE;
    const LENGTH: usize = 1usize;
    type FixedType = i32;
    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
        (Member0::test_assoc_fn(arg.0),)
    }
    fn test_self_fn(&self) -> Result<(), ()> {
        self.0.test_self_fn()?;
        Ok(())
    }
}
impl<Member0, Member1> Trait for (Member0, Member1)
where
    Member0: Trait<FixedType = i32>,
    Member1: Trait<FixedType = i32>,
{
    type Ret = (Member0::Ret, Member1::Ret);
    type Arg = (Member0::Arg, Member1::Arg);
    const VALUE: i32 = Member0::VALUE + Member1::VALUE;
    const LENGTH: usize = 2usize;
    type FixedType = i32;
    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
        (Member0::test_assoc_fn(arg.0), Member1::test_assoc_fn(arg.1))
    }
    fn test_self_fn(&self) -> Result<(), ()> {
        self.0.test_self_fn()?;
        self.1.test_self_fn()?;
        Ok(())
    }
}
impl<Member0, Member1, Member2> Trait for (Member0, Member1, Member2)
where
    Member0: Trait<FixedType = i32>,
    Member1: Trait<FixedType = i32>,
    Member2: Trait<FixedType = i32>,
{
    type Ret = (Member0::Ret, Member1::Ret, Member2::Ret);
    type Arg = (Member0::Arg, Member1::Arg, Member2::Arg);
    const VALUE: i32 = Member0::VALUE + Member1::VALUE + Member2::VALUE;
    const LENGTH: usize = 3usize;
    type FixedType = i32;
    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
        (
            Member0::test_assoc_fn(arg.0),
            Member1::test_assoc_fn(arg.1),
            Member2::test_assoc_fn(arg.2),
        )
    }
    fn test_self_fn(&self) -> Result<(), ()> {
        self.0.test_self_fn()?;
        self.1.test_self_fn()?;
        self.2.test_self_fn()?;
        Ok(())
    }
}
impl<Member0, Member1, Member2, Member3> Trait for (Member0, Member1, Member2, Member3)
where
    Member0: Trait<FixedType = i32>,
    Member1: Trait<FixedType = i32>,
    Member2: Trait<FixedType = i32>,
    Member3: Trait<FixedType = i32>,
{
    type Ret = (Member0::Ret, Member1::Ret, Member2::Ret, Member3::Ret);
    type Arg = (Member0::Arg, Member1::Arg, Member2::Arg, Member3::Arg);
    const VALUE: i32 = Member0::VALUE + Member1::VALUE + Member2::VALUE + Member3::VALUE;
    const LENGTH: usize = 4usize;
    type FixedType = i32;
    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
        (
            Member0::test_assoc_fn(arg.0),
            Member1::test_assoc_fn(arg.1),
            Member2::test_assoc_fn(arg.2),
            Member3::test_assoc_fn(arg.3),
        )
    }
    fn test_self_fn(&self) -> Result<(), ()> {
        self.0.test_self_fn()?;
        self.1.test_self_fn()?;
        self.2.test_self_fn()?;
        self.3.test_self_fn()?;
        Ok(())
    }
}
impl<Member0, Member1, Member2, Member3, Member4> Trait
    for (Member0, Member1, Member2, Member3, Member4)
where
    Member0: Trait<FixedType = i32>,
    Member1: Trait<FixedType = i32>,
    Member2: Trait<FixedType = i32>,
    Member3: Trait<FixedType = i32>,
    Member4: Trait<FixedType = i32>,
{
    type Ret = (
        Member0::Ret,
        Member1::Ret,
        Member2::Ret,
        Member3::Ret,
        Member4::Ret,
    );
    type Arg = (
        Member0::Arg,
        Member1::Arg,
        Member2::Arg,
        Member3::Arg,
        Member4::Arg,
    );
    const VALUE: i32 =
        Member0::VALUE + Member1::VALUE + Member2::VALUE + Member3::VALUE + Member4::VALUE;
    const LENGTH: usize = 5usize;
    type FixedType = i32;
    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
        (
            Member0::test_assoc_fn(arg.0),
            Member1::test_assoc_fn(arg.1),
            Member2::test_assoc_fn(arg.2),
            Member3::test_assoc_fn(arg.3),
            Member4::test_assoc_fn(arg.4),
        )
    }
    fn test_self_fn(&self) -> Result<(), ()> {
        self.0.test_self_fn()?;
        self.1.test_self_fn()?;
        self.2.test_self_fn()?;
        self.3.test_self_fn()?;
        self.4.test_self_fn()?;
        Ok(())
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5> Trait
    for (Member0, Member1, Member2, Member3, Member4, Member5)
where
    Member0: Trait<FixedType = i32>,
    Member1: Trait<FixedType = i32>,
    Member2: Trait<FixedType = i32>,
    Member3: Trait<FixedType = i32>,
    Member4: Trait<FixedType = i32>,
    Member5: Trait<FixedType = i32>,
{
    type Ret = (
        Member0::Ret,
        Member1::Ret,
        Member2::Ret,
        Member3::Ret,
        Member4::Ret,
        Member5::Ret,
    );
    type Arg = (
        Member0::Arg,
        Member1::Arg,
        Member2::Arg,
        Member3::Arg,
        Member4::Arg,
        Member5::Arg,
    );
    const VALUE: i32 = Member0::VALUE
        + Member1::VALUE
        + Member2::VALUE
        + Member3::VALUE
        + Member4::VALUE
        + Member5::VALUE;
    const LENGTH: usize = 6usize;
    type FixedType = i32;
    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
        (
            Member0::test_assoc_fn(arg.0),
            Member1::test_assoc_fn(arg.1),
            Member2::test_assoc_fn(arg.2),
            Member3::test_assoc_fn(arg.3),
            Member4::test_assoc_fn(arg.4),
            Member5::test_assoc_fn(arg.5),
        )
    }
    fn test_self_fn(&self) -> Result<(), ()> {
        self.0.test_self_fn()?;
        self.1.test_self_fn()?;
        self.2.test_self_fn()?;
        self.3.test_self_fn()?;
        self.4.test_self_fn()?;
        self.5.test_self_fn()?;
        Ok(())
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5, Member6> Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
    )
where
    Member0: Trait<FixedType = i32>,
    Member1: Trait<FixedType = i32>,
    Member2: Trait<FixedType = i32>,
    Member3: Trait<FixedType = i32>,
    Member4: Trait<FixedType = i32>,
    Member5: Trait<FixedType = i32>,
    Member6: Trait<FixedType = i32>,
{
    type Ret = (
        Member0::Ret,
        Member1::Ret,
        Member2::Ret,
        Member3::Ret,
        Member4::Ret,
        Member5::Ret,
        Member6::Ret,
    );
    type Arg = (
        Member0::Arg,
        Member1::Arg,
        Member2::Arg,
        Member3::Arg,
        Member4::Arg,
        Member5::Arg,
        Member6::Arg,
    );
    const VALUE: i32 = Member0::VALUE
        + Member1::VALUE
        + Member2::VALUE
        + Member3::VALUE
        + Member4::VALUE
        + Member5::VALUE
        + Member6::VALUE;
    const LENGTH: usize = 7usize;
    type FixedType = i32;
    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
        (
            Member0::test_assoc_fn(arg.0),
            Member1::test_assoc_fn(arg.1),
            Member2::test_assoc_fn(arg.2),
            Member3::test_assoc_fn(arg.3),
            Member4::test_assoc_fn(arg.4),
            Member5::test_assoc_fn(arg.5),
            Member6::test_assoc_fn(arg.6),
        )
    }
    fn test_self_fn(&self) -> Result<(), ()> {
        self.0.test_self_fn()?;
        self.1.test_self_fn()?;
        self.2.test_self_fn()?;
        self.3.test_self_fn()?;
        self.4.test_self_fn()?;
        self.5.test_self_fn()?;
        self.6.test_self_fn()?;
        Ok(())
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5, Member6, Member7> Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
    )
where
    Member0: Trait<FixedType = i32>,
    Member1: Trait<FixedType = i32>,
    Member2: Trait<FixedType = i32>,
    Member3: Trait<FixedType = i32>,
    Member4: Trait<FixedType = i32>,
    Member5: Trait<FixedType = i32>,
    Member6: Trait<FixedType = i32>,
    Member7: Trait<FixedType = i32>,
{
    type Ret = (
        Member0::Ret,
        Member1::Ret,
        Member2::Ret,
        Member3::Ret,
        Member4::Ret,
        Member5::Ret,
        Member6::Ret,
        Member7::Ret,
    );
    type Arg = (
        Member0::Arg,
        Member1::Arg,
        Member2::Arg,
        Member3::Arg,
        Member4::Arg,
        Member5::Arg,
        Member6::Arg,
        Member7::Arg,
    );
    const VALUE: i32 = Member0::VALUE
        + Member1::VALUE
        + Member2::VALUE
        + Member3::VALUE
        + Member4::VALUE
        + Member5::VALUE
        + Member6::VALUE
        + Member7::VALUE;
    const LENGTH: usize = 8usize;
    type FixedType = i32;
    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
        (
            Member0::test_assoc_fn(arg.0),
            Member1::test_assoc_fn(arg.1),
            Member2::test_assoc_fn(arg.2),
            Member3::test_assoc_fn(arg.3),
            Member4::test_assoc_fn(arg.4),
            Member5::test_assoc_fn(arg.5),
            Member6::test_assoc_fn(arg.6),
            Member7::test_assoc_fn(arg.7),
        )
    }
    fn test_self_fn(&self) -> Result<(), ()> {
        self.0.test_self_fn()?;
        self.1.test_self_fn()?;
        self.2.test_self_fn()?;
        self.3.test_self_fn()?;
        self.4.test_self_fn()?;
        self.5.test_self_fn()?;
        self.6.test_self_fn()?;
        self.7.test_self_fn()?;
        Ok(())
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5, Member6, Member7, Member8> Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
    )
where
    Member0: Trait<FixedType = i32>,
    Member1: Trait<FixedType = i32>,
    Member2: Trait<FixedType = i32>,
    Member3: Trait<FixedType = i32>,
    Member4: Trait<FixedType = i32>,
    Member5: Trait<FixedType = i32>,
    Member6: Trait<FixedType = i32>,
    Member7: Trait<FixedType = i32>,
    Member8: Trait<FixedType = i32>,
{
    type Ret = (
        Member0::Ret,
        Member1::Ret,
        Member2::Ret,
        Member3::Ret,
        Member4::Ret,
        Member5::Ret,
        Member6::Ret,
        Member7::Ret,
        Member8::Ret,
    );
    type Arg = (
        Member0::Arg,
        Member1::Arg,
        Member2::Arg,
        Member3::Arg,
        Member4::Arg,
        Member5::Arg,
        Member6::Arg,
        Member7::Arg,
        Member8::Arg,
    );
    const VALUE: i32 = Member0::VALUE
        + Member1::VALUE
        + Member2::VALUE
        + Member3::VALUE
        + Member4::VALUE
        + Member5::VALUE
        + Member6::VALUE
        + Member7::VALUE
        + Member8::VALUE;
    const LENGTH: usize = 9usize;
    type FixedType = i32;
    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
        (
            Member0::test_assoc_fn(arg.0),
            Member1::test_assoc_fn(arg.1),
            Member2::test_assoc_fn(arg.2),
            Member3::test_assoc_fn(arg.3),
            Member4::test_assoc_fn(arg.4),
            Member5::test_assoc_fn(arg.5),
            Member6::test_assoc_fn(arg.6),
            Member7::test_assoc_fn(arg.7),
            Member8::test_assoc_fn(arg.8),
        )
    }
    fn test_self_fn(&self) -> Result<(), ()> {
        self.0.test_self_fn()?;
        self.1.test_self_fn()?;
        self.2.test_self_fn()?;
        self.3.test_self_fn()?;
        self.4.test_self_fn()?;
        self.5.test_self_fn()?;
        self.6.test_self_fn()?;
        self.7.test_self_fn()?;
        self.8.test_self_fn()?;
        Ok(())
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5, Member6, Member7, Member8, Member9> Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
    )
where
    Member0: Trait<FixedType = i32>,
    Member1: Trait<FixedType = i32>,
    Member2: Trait<FixedType = i32>,
    Member3: Trait<FixedType = i32>,
    Member4: Trait<FixedType = i32>,
    Member5: Trait<FixedType = i32>,
    Member6: Trait<FixedType = i32>,
    Member7: Trait<FixedType = i32>,
    Member8: Trait<FixedType = i32>,
    Member9: Trait<FixedType = i32>,
{
    type Ret = (
        Member0::Ret,
        Member1::Ret,
        Member2::Ret,
        Member3::Ret,
        Member4::Ret,
        Member5::Ret,
        Member6::Ret,
        Member7::Ret,
        Member8::Ret,
        Member9::Ret,
    );
    type Arg = (
        Member0::Arg,
        Member1::Arg,
        Member2::Arg,
        Member3::Arg,
        Member4::Arg,
        Member5::Arg,
        Member6::Arg,
        Member7::Arg,
        Member8::Arg,
        Member9::Arg,
    );
    const VALUE: i32 = Member0::VALUE
        + Member1::VALUE
        + Member2::VALUE
        + Member3::VALUE
        + Member4::VALUE
        + Member5::VALUE
        + Member6::VALUE
        + Member7::VALUE
        + Member8::VALUE
        + Member9::VALUE;
    const LENGTH: usize = 10usize;
    type FixedType = i32;
    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
        (
            Member0::test_assoc_fn(arg.0),
            Member1::test_assoc_fn(arg.1),
            Member2::test_assoc_fn(arg.2),
            Member3::test_assoc_fn(arg.3),
            Member4::test_assoc_fn(arg.4),
            Member5::test_assoc_fn(arg.5),
            Member6::test_assoc_fn(arg.6),
            Member7::test_assoc_fn(arg.7),
            Member8::test_assoc_fn(arg.8),
            Member9::test_assoc_fn(arg.9),
        )
    }
    fn test_self_fn(&self) -> Result<(), ()> {
        self.0.test_self_fn()?;
        self.1.test_self_fn()?;
        self.2.test_self_fn()?;
        self.3.test_self_fn()?;
        self.4.test_self_fn()?;
        self.5.test_self_fn()?;
        self.6.test_self_fn()?;
        self.7.test_self_fn()?;
        self.8.test_self_fn()?;
        self.9.test_self_fn()?;
        Ok(())
    }
}
impl<
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
    > Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
    )
where
    Member0: Trait<FixedType = i32>,
    Member1: Trait<FixedType = i32>,
    Member2: Trait<FixedType = i32>,
    Member3: Trait<FixedType = i32>,
    Member4: Trait<FixedType = i32>,
    Member5: Trait<FixedType = i32>,
    Member6: Trait<FixedType = i32>,
    Member7: Trait<FixedType = i32>,
    Member8: Trait<FixedType = i32>,
    Member9: Trait<FixedType = i32>,
    Member10: Trait<FixedType = i32>,
{
    type Ret = (
        Member0::Ret,
        Member1::Ret,
        Member2::Ret,
        Member3::Ret,
        Member4::Ret,
        Member5::Ret,
        Member6::Ret,
        Member7::Ret,
        Member8::Ret,
        Member9::Ret,
        Member10::Ret,
    );
    type Arg = (
        Member0::Arg,
        Member1::Arg,
        Member2::Arg,
        Member3::Arg,
        Member4::Arg,
        Member5::Arg,
        Member6::Arg,
        Member7::Arg,
        Member8::Arg,
        Member9::Arg,
        Member10::Arg,
    );
    const VALUE: i32 = Member0::VALUE
        + Member1::VALUE
        + Member2::VALUE
        + Member3::VALUE
        + Member4::VALUE
        + Member5::VALUE
        + Member6::VALUE
        + Member7::VALUE
        + Member8::VALUE
        + Member9::VALUE
        + Member10::VALUE;
    const LENGTH: usize = 11usize;
    type FixedType = i32;
    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
        (
            Member0::test_assoc_fn(arg.0),
            Member1::test_assoc_fn(arg.1),
            Member2::test_assoc_fn(arg.2),
            Member3::test_assoc_fn(arg.3),
            Member4::test_assoc_fn(arg.4),
            Member5::test_assoc_fn(arg.5),
            Member6::test_assoc_fn(arg.6),
            Member7::test_assoc_fn(arg.7),
            Member8::test_assoc_fn(arg.8),
            Member9::test_assoc_fn(arg.9),
            Member10::test_assoc_fn(arg.10),
        )
    }
    fn test_self_fn(&self) -> Result<(), ()> {
        self.0.test_self_fn()?;
        self.1.test_self_fn()?;
        self.2.test_self_fn()?;
        self.3.test_self_fn()?;
        self.4.test_self_fn()?;
        self.5.test_self_fn()?;
        self.6.test_self_fn()?;
        self.7.test_self_fn()?;
        self.8.test_self_fn()?;
        self.9.test_self_fn()?;
        self.10.test_self_fn()?;
        Ok(())
    }
}
impl<
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
    > Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
    )
where
    Member0: Trait<FixedType = i32>,
    Member1: Trait<FixedType = i32>,
    Member2: Trait<FixedType = i32>,
    Member3: Trait<FixedType = i32>,
    Member4: Trait<FixedType = i32>,
    Member5: Trait<FixedType = i32>,
    Member6: Trait<FixedType = i32>,
    Member7: Trait<FixedType = i32>,
    Member8: Trait<FixedType = i32>,
    Member9: Trait<FixedType = i32>,
    Member10: Trait<FixedType = i32>,
    Member11: Trait<FixedType = i32>,
{
    type Ret = (
        Member0::Ret,
        Member1::Ret,
        Member2::Ret,
        Member3::Ret,
        Member4::Ret,
        Member5::Ret,
        Member6::Ret,
        Member7::Ret,
        Member8::Ret,
        Member9::Ret,
        Member10::Ret,
        Member11::Ret,
    );
    type Arg = (
        Member0::Arg,
        Member1::Arg,
        Member2::Arg,
        Member3::Arg,
        Member4::Arg,
        Member5::Arg,
        Member6::Arg,
        Member7::Arg,
        Member8::Arg,
        Member9::Arg,
        Member10::Arg,
        Member11::Arg,
    );
    const VALUE: i32 = Member0::VALUE
        + Member1::VALUE
        + Member2::VALUE
        + Member3::VALUE
        + Member4::VALUE
        + Member5::VALUE
        + Member6::VALUE
        + Member7::VALUE
        + Member8::VALUE
        + Member9::VALUE
        + Member10::VALUE
        + Member11::VALUE;
    const LENGTH: usize = 12usize;
    type FixedType = i32;
    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
        (
            Member0::test_assoc_fn(arg.0),
            Member1::test_assoc_fn(arg.1),
            Member2::test_assoc_fn(arg.2),
            Member3::test_assoc_fn(arg.3),
            Member4::test_assoc_fn(arg.4),
            Member5::test_assoc_fn(arg.5),
            Member6::test_assoc_fn(arg.6),
            Member7::test_assoc_fn(arg.7),
            Member8::test_assoc_fn(arg.8),
            Member9::test_assoc_fn(arg.9),
            Member10::test_assoc_fn(arg.10),
            Member11::test_assoc_fn(arg.11),
        )
    }
    fn test_self_fn(&self) -> Result<(), ()> {
        self.0.test_self_fn()?;
        self.1.test_self_fn()?;
        self.2.test_self_fn()?;
        self.3.test_self_fn()?;
        self.4.test_self_fn()?;
        self.5.test_self_fn()?;
        self.6.test_self_fn()?;
        self.7.test_self_fn()?;
        self.8.test_self_fn()?;
        self.9.test_self_fn()?;
        self.10.test_self_fn()?;
        self.11.test_self_fn()?;
        Ok(())
    }
}
impl<
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
        Member12,
    > Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
        Member12,
    )
where
    Member0: Trait<FixedType = i32>,
    Member1: Trait<FixedType = i32>,
    Member2: Trait<FixedType = i32>,
    Member3: Trait<FixedType = i32>,
    Member4: Trait<FixedType = i32>,
    Member5: Trait<FixedType = i32>,
    Member6: Trait<FixedType = i32>,
    Member7: Trait<FixedType = i32>,
    Member8: Trait<FixedType = i32>,
    Member9: Trait<FixedType = i32>,
    Member10: Trait<FixedType = i32>,
    Member11: Trait<FixedType = i32>,
    Member12: Trait<FixedType = i32>,
{
    type Ret = (
        Member0::Ret,
        Member1::Ret,
        Member2::Ret,
        Member3::Ret,
        Member4::Ret,
        Member5::Ret,
        Member6::Ret,
        Member7::Ret,
        Member8::Ret,
        Member9::Ret,
        Member10::Ret,
        Member11::Ret,
        Member12::Ret,
    );
    type Arg = (
        Member0::Arg,
        Member1::Arg,
        Member2::Arg,
        Member3::Arg,
        Member4::Arg,
        Member5::Arg,
        Member6::Arg,
        Member7::Arg,
        Member8::Arg,
        Member9::Arg,
        Member10::Arg,
        Member11::Arg,
        Member12::Arg,
    );
    const VALUE: i32 = Member0::VALUE
        + Member1::VALUE
        + Member2::VALUE
        + Member3::VALUE
        + Member4::VALUE
        + Member5::VALUE
        + Member6::VALUE
        + Member7::VALUE
        + Member8::VALUE
        + Member9::VALUE
        + Member10::VALUE
        + Member11::VALUE
        + Member12::VALUE;
    const LENGTH: usize = 13usize;
    type FixedType = i32;
    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
        (
            Member0::test_assoc_fn(arg.0),
            Member1::test_assoc_fn(arg.1),
            Member2::test_assoc_fn(arg.2),
            Member3::test_assoc_fn(arg.3),
            Member4::test_assoc_fn(arg.4),
            Member5::test_assoc_fn(arg.5),
            Member6::test_assoc_fn(arg.6),
            Member7::test_assoc_fn(arg.7),
            Member8::test_assoc_fn(arg.8),
            Member9::test_assoc_fn(arg.9),
            Member10::test_assoc_fn(arg.10),
            Member11::test_assoc_fn(arg.11),
            Member12::test_assoc_fn(arg.12),
        )
    }
    fn test_self_fn(&self) -> Result<(), ()> {
        self.0.test_self_fn()?;
        self.1.test_self_fn()?;
        self.2.test_self_fn()?;
        self.3.test_self_fn()?;
        self.4.test_self_fn()?;
        self.5.test_self_fn()?;
        self.6.test_self_fn()?;
        self.7.test_self_fn()?;
        self.8.test_self_fn()?;
        self.9.test_self_fn()?;
        self.10.test_self_fn()?;
        self.11.test_self_fn()?;
        self.12.test_self_fn()?;
        Ok(())
    }
}
impl<
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
        Member12,
        Member13,
    > Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
        Member12,
        Member13,
    )
where
    Member0: Trait<FixedType = i32>,
    Member1: Trait<FixedType = i32>,
    Member2: Trait<FixedType = i32>,
    Member3: Trait<FixedType = i32>,
    Member4: Trait<FixedType = i32>,
    Member5: Trait<FixedType = i32>,
    Member6: Trait<FixedType = i32>,
    Member7: Trait<FixedType = i32>,
    Member8: Trait<FixedType = i32>,
    Member9: Trait<FixedType = i32>,
    Member10: Trait<FixedType = i32>,
    Member11: Trait<FixedType = i32>,
    Member12: Trait<FixedType = i32>,
    Member13: Trait<FixedType = i32>,
{
    type Ret = (
        Member0::Ret,
        Member1::Ret,
        Member2::Ret,
        Member3::Ret,
        Member4::Ret,
        Member5::Ret,
        Member6::Ret,
        Member7::Ret,
        Member8::Ret,
        Member9::Ret,
        Member10::Ret,
        Member11::Ret,
        Member12::Ret,
        Member13::Ret,
    );
    type Arg = (
        Member0::Arg,
        Member1::Arg,
        Member2::Arg,
        Member3::Arg,
        Member4::Arg,
        Member5::Arg,
        Member6::Arg,
        Member7::Arg,
        Member8::Arg,
        Member9::Arg,
        Member10::Arg,
        Member11::Arg,
        Member12::Arg,
        Member13::Arg,
    );
    const VALUE: i32 = Member0::VALUE
        + Member1::VALUE
        + Member2::VALUE
        + Member3::VALUE
        + Member4::VALUE
        + Member5::VALUE
        + Member6::VALUE
        + Member7::VALUE
        + Member8::VALUE
        + Member9::VALUE
        + Member10::VALUE
        + Member11::VALUE
        + Member12::VALUE
        + Member13::VALUE;
    const LENGTH: usize = 14usize;
    type FixedType = i32;
    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
        (
            Member0::test_assoc_fn(arg.0),
            Member1::test_assoc_fn(arg.1),
            Member2::test_assoc_fn(arg.2),
            Member3::test_assoc_fn(arg.3),
            Member4::test_assoc_fn(arg.4),
            Member5::test_assoc_fn(arg.5),
            Member6::test_assoc_fn(arg.6),
            Member7::test_assoc_fn(arg.7),
            Member8::test_assoc_fn(arg.8),
            Member9::test_assoc_fn(arg.9),
            Member10::test_assoc_fn(arg.10),
            Member11::test_assoc_fn(arg.11),
            Member12::test_assoc_fn(arg.12),
            Member13::test_assoc_fn(arg.13),
        )
    }
    fn test_self_fn(&self) -> Result<(), ()> {
        self.0.test_self_fn()?;
        self.1.test_self_fn()?;
        self.2.test_self_fn()?;
        self.3.test_self_fn()?;
        self.4.test_self_fn()?;
        self.5.test_self_fn()?;
        self.6.test_self_fn()?;
        self.7.test_self_fn()?;
        self.8.test_self_fn()?;
        self.9.test_self_fn()?;
        self.10.test_self_fn()?;
        self.11.test_self_fn()?;
        self.12.test_self_fn()?;
        self.13.test_self_fn()?;
        Ok(())
    }
}
impl<
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
        Member12,
        Member13,
        Member14,
    > Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
        Member12,
        Member13,
        Member14,
    )
where
    Member0: Trait<FixedType = i32>,
    Member1: Trait<FixedType = i32>,
    Member2: Trait<FixedType = i32>,
    Member3: Trait<FixedType = i32>,
    Member4: Trait<FixedType = i32>,
    Member5: Trait<FixedType = i32>,
    Member6: Trait<FixedType = i32>,
    Member7: Trait<FixedType = i32>,
    Member8: Trait<FixedType = i32>,
    Member9: Trait<FixedType = i32>,
    Member10: Trait<FixedType = i32>,
    Member11: Trait<FixedType = i32>,
    Member12: Trait<FixedType = i32>,
    Member13: Trait<FixedType = i32>,
    Member14: Trait<FixedType = i32>,
{
    type Ret = (
        Member0::Ret,
        Member1::Ret,
        Member2::Ret,
        Member3::Ret,
        Member4::Ret,
        Member5::Ret,
        Member6::Ret,
        Member7::Ret,
        Member8::Ret,
        Member9::Ret,
        Member10::Ret,
        Member11::Ret,
        Member12::Ret,
        Member13::Ret,
        Member14::Ret,
    );
    type Arg = (
        Member0::Arg,
        Member1::Arg,
        Member2::Arg,
        Member3::Arg,
        Member4::Arg,
        Member5::Arg,
        Member6::Arg,
        Member7::Arg,
        Member8::Arg,
        Member9::Arg,
        Member10::Arg,
        Member11::Arg,
        Member12::Arg,
        Member13::Arg,
        Member14::Arg,
    );
    const VALUE: i32 = Member0::VALUE
        + Member1::VALUE
        + Member2::VALUE
        + Member3::VALUE
        + Member4::VALUE
        + Member5::VALUE
        + Member6::VALUE
        + Member7::VALUE
        + Member8::VALUE
        + Member9::VALUE
        + Member10::VALUE
        + Member11::VALUE
        + Member12::VALUE
        + Member13::VALUE
        + Member14::VALUE;
    const LENGTH: usize = 15usize;
    type FixedType = i32;
    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
        (
            Member0::test_assoc_fn(arg.0),
            Member1::test_assoc_fn(arg.1),
            Member2::test_assoc_fn(arg.2),
            Member3::test_assoc_fn(arg.3),
            Member4::test_assoc_fn(arg.4),
            Member5::test_assoc_fn(arg.5),
            Member6::test_assoc_fn(arg.6),
            Member7::test_assoc_fn(arg.7),
            Member8::test_assoc_fn(arg.8),
            Member9::test_assoc_fn(arg.9),
            Member10::test_assoc_fn(arg.10),
            Member11::test_assoc_fn(arg.11),
            Member12::test_assoc_fn(arg.12),
            Member13::test_assoc_fn(arg.13),
            Member14::test_assoc_fn(arg.14),
        )
    }
    fn test_self_fn(&self) -> Result<(), ()> {
        self.0.test_self_fn()?;
        self.1.test_self_fn()?;
        self.2.test_self_fn()?;
        self.3.test_self_fn()?;
        self.4.test_self_fn()?;
        self.5.test_self_fn()?;
        self.6.test_self_fn()?;
        self.7.test_self_fn()?;
        self.8.test_self_fn()?;
        self.9.test_self_fn()?;
        self.10.test_self_fn()?;
        self.11.test_self_fn()?;
        self.12.test_self_fn()?;
        self.13.test_self_fn()?;
        self.14.test_self_fn()?;
        Ok(())
    }
}
impl<
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
        Member12,
        Member13,
        Member14,
        Member15,
    > Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
        Member12,
        Member13,
        Member14,
        Member15,
    )
where
    Member0: Trait<FixedType = i32>,
    Member1: Trait<FixedType = i32>,
    Member2: Trait<FixedType = i32>,
    Member3: Trait<FixedType = i32>,
    Member4: Trait<FixedType = i32>,
    Member5: Trait<FixedType = i32>,
    Member6: Trait<FixedType = i32>,
    Member7: Trait<FixedType = i32>,
    Member8: Trait<FixedType = i32>,
    Member9: Trait<FixedType = i32>,
    Member10: Trait<FixedType = i32>,
    Member11: Trait<FixedType = i32>,
    Member12: Trait<FixedType = i32>,
    Member13: Trait<FixedType = i32>,
    Member14: Trait<FixedType = i32>,
    Member15: Trait<FixedType = i32>,
{
    type Ret = (
        Member0::Ret,
        Member1::Ret,
        Member2::Ret,
        Member3::Ret,
        Member4::Ret,
        Member5::Ret,
        Member6::Ret,
        Member7::Ret,
        Member8::Ret,
        Member9::Ret,
        Member10::Ret,
        Member11::Ret,
        Member12::Ret,
        Member13::Ret,
        Member14::Ret,
        Member15::Ret,
    );
    type Arg = (
        Member0::Arg,
        Member1::Arg,
        Member2::Arg,
        Member3::Arg,
        Member4::Arg,
        Member5::Arg,
        Member6::Arg,
        Member7::Arg,
        Member8::Arg,
        Member9::Arg,
        Member10::Arg,
        Member11::Arg,
        Member12::Arg,
        Member13::Arg,
        Member14::Arg,
        Member15::Arg,
    );
    const VALUE: i32 = Member0::VALUE
        + Member1::VALUE
        + Member2::VALUE
        + Member3::VALUE
        + Member4::VALUE
        + Member5::VALUE
        + Member6::VALUE
        + Member7::VALUE
        + Member8::VALUE
        + Member9::VALUE
        + Member10::VALUE
        + Member11::VALUE
        + Member12::VALUE
        + Member13::VALUE
        + Member14::VALUE
        + Member15::VALUE;
    const LENGTH: usize = 16usize;
    type FixedType = i32;
    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
        (
            Member0::test_assoc_fn(arg.0),
            Member1::test_assoc_fn(arg.1),
            Member2::test_assoc_fn(arg.2),
            Member3::test_assoc_fn(arg.3),
            Member4::test_assoc_fn(arg.4),
            Member5::test_assoc_fn(arg.5),
            Member6::test_assoc_fn(arg.6),
            Member7::test_assoc_fn(arg.7),
            Member8::test_assoc_fn(arg.8),
            Member9::test_assoc_fn(arg.9),
            Member10::test_assoc_fn(arg.10),
            Member11::test_assoc_fn(arg.11),
            Member12::test_assoc_fn(arg.12),
            Member13::test_assoc_fn(arg.13),
            Member14::test_assoc_fn(arg.14),
            Member15::test_assoc_fn(arg.15),
        )
    }
    fn test_self_fn(&self) -> Result<(), ()> {
        self.0.test_self_fn()?;
        self.1.test_self_fn()?;
        self.2.test_self_fn()?;
        self.3.test_self_fn()?;
        self.4.test_self_fn()?;
        self.5.test_self_fn()?;
        self.6.test_self_fn()?;
        self.7.test_self_fn()?;
        self.8.test_self_fn()?;
        self.9.test_self_fn()?;
        self.10.test_self_fn()?;
        self.11.test_self_fn()?;
        self.12.test_self_fn()?;
        self.13.test_self_fn()?;
        self.14.test_self_fn()?;
        self.15.test_self_fn()?;
        Ok(())
    }
}
