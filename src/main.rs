use my_proc_macro_derive::StructInfo;
use proc_macro_demo::StructInfo;
use my_proc_macro_derive::aop;
use my_proc_macro_derive::add;

// ------------------  派生宏 begin -------------
#[derive(StructInfo)]
struct Person{
    name: String,
    age: i32,
}

#[derive(StructInfo)]
struct Animal{
    name: String,
    age: i32,
}
// ------------------  派生宏 end -------------

// ------------------ 属性宏 begin ------------
#[aop]
fn my_fn() {
    println!("In the function");
}
// ------------------ 属性宏 end ------------

fn main() {
    // ------------------  派生宏 begin -------------
    let xy = Person{name: String::from("XY"), age: 100};
    println!("{}", xy.i_name()); // Person

    let xb = Animal{name: String::from("XB"), age: 1};
    println!("{}", xb.i_name()); // Animal
    // ------------------  派生宏 end -------------

    // ------------------ 属性宏 begin ------------
    my_fn();
    // ------------------ 属性宏 end ------------

    // ------------ 函数宏 begin
    let r = add!(10, 20);
    println!("add = {}", r);
    // ------------ 函数宏 end

}
