// Borrowing and References
// references let you use a value without taking ownership. this is called borrowing
// we have two kind of ref 1) &A is a shared(immutabale) ref you can only read , not change.
// 2) mutable  &mutA you can read or change

fn main(){
   /*  let name = String::from("Bhavesh");
    let user = name;
    println!("{}", user) */ // give us a error becuase we transffer ownership is not possible

    // using ref to solve this we give read or edit permission acording to ref type mut or unmut 

    // mutable ref allows you to modify

    let mut name = String::from("Bhavesh");
    let user = &mut name;
    user.push_str(" kumar");
    println!("{}", name);
    
    // immutable ref allow only read not modify

    let who = String::from("Raj");

    let user1 = &who;
    let user2 = &who;

    println!("{}", user1);
    println!("{}", user2);
}

