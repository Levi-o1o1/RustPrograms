fn main(){
    greet("Mukesh");
    println!("{}",adding(43, 23));
}

fn greet(name: &str){
    println!("hello Mr. {}", name)
}

fn adding(a: i32, b: i32) ->i32{
    a + b
}