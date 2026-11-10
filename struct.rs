// struct(structure) is a way to group multiple realted pieces of data into one single type.
// we can create a mixed var something like that where we have int , stirng , bool or etc..

struct Students{
    name: String,
    age: u8,
    roll_no: u32,
    marks: f32,
}
fn main(){
    
    let stu1 = Students{                    // instance of the students struct  // let mut stu1 if we want mutable struct
        name: String::from("Raj"),
        age: 19,
        roll_no: 101,
        marks: 55.43,
    };

    println!("name is :{} age is : {} roll no is: {} marks is : {}", stu1.name, stu1.age, stu1.roll_no, stu1.marks);


}