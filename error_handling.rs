fn find_user(id: u32) -> Option<String>{
    if id == 1{
        Some(String::from("Ayushi"))

    }else{
        None
    }

}
fn divide(a: f64, b: f64) -> Result<f64, String>{
    if b == 0.0{
        Err(String::from("cannot divide by zero "))
    }else {
        Ok(a/b)
    }
}

fn main(){
    let result : Option<String> = find_user( 1);  // if id is 2 its show not user found 
    match result{
        Some(name)=> println!("Found {}", name),
        None => println!("no user found"),
    }

    match divide(10.0,  2.0){
        Ok(value) => println!("the result is : {}", value),
        Err(e) => println!("Error: {}",e),

    }
}