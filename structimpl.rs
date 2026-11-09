// struct with impl 
struct Student{
    name: String,
    age: u8,

}

impl Student{

    fn display(&self){
        println!("name: {}", self.name);
        println!("age: {}", self.age);

    }
}

fn main(){
    let student1 = Student{
        name: String::from("Mukesh"),
        age: 19,
    };

    student1.display();
}