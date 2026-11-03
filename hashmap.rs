use std::collections::HashMap; // standard library import


fn main(){

    let mut ages: HashMap<String, u32> = HashMap::new();
    (ages).insert(String::from("ayush"), 29);
    (ages).insert(String::from("rai"), 48);

    if let Some(age) = ages.get("ayush"){
        println!("ayush is {}", age)
    }
    for(name, age) in &ages{
        println!("{} is {}", name, age)
    }

}