fn find_user(id: u32) -> Option<String>{
    if id == 1{
        Some(String::from("Ayushi"))

    }else{
        None
    }

}

fn main(){
    let result : Option<String> = find_user( 1);  // if id is 2 its show not user found 
    match result{
        Some(name)=> println!("Found {}", name),
        None => println!("no user found")
    }
}