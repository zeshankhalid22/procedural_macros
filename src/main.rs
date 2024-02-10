#![allow(warnings)]

use derive_describe::Describe;
#[derive(Describe)]
struct MyStruct {
    name: String,
    age: i32,
    my_enum: MyEnum
}

#[derive(Describe)]
enum MyEnum {
    Rupee,
    Dollar,
}

#[derive(Describe)]
struct MyTupleStruct(u32, String, i8);



fn main() {
    MyStruct::describe();
    MyTupleStruct::describe();
    MyEnum::describe();
}
