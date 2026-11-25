fn main(){

    let number: i32 = 6;

    let label : &str = if number % 2 == 0 {"even"} else {"odd"};

    println!("{}", label);

    println!("{:?}",ifcon())
}

fn ifcon(){

    let num: i32 = 5;

    if num % 2 == 0 {
        
        println!("the number is even ")
    }else{
        println!("the number is odd")
       
       
        
    
    }
}