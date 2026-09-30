// TODO: this is an example of an orphan rule violation.
//  We're implementing a foreign trait (`PartialEq`, from `std`) on
//  a foreign type (`u32`, from `std`).
//  Look at the compiler error to get familiar with what it looks like.
//  Then delete the code below and move on to the next exercise.

pub struct MyLocalStruct;

// VALID: Display is foreign, but MyLocalStruct is local.
impl std::fmt::Display for MyLocalStruct {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

// VALID: From is foreign, MyLocalStruct is local after Vec<T> but Vec<T> covers T
impl<T> From<MyLocalStruct> for Vec<T> {
    fn from(_: MyLocalStruct) -> Self {
        todo!()
    }
}

// INVALID: From is foreign, MyLocalStruct is local after Box<T> but Box is fundamental type so Box<T> doesn't cover T.
// impl<T> From<MyLocalStruct> for Box<T> {
//     fn from(_: MyLocalStruct) -> Self {
//         todo!()
//     }
// }

// VALID: From is foreign, MyLocalStruct is local before Box<T>, so no matter Box<T> covers T or not.
impl<T> From<Box<T>> for MyLocalStruct {
    fn from(_: Box<T>) -> Self {
        todo!()
    }
}

// impl PartialEq for u32 {
//     fn eq(&self, _other: &Self) -> bool {
//         todo!()
//     }
// }
