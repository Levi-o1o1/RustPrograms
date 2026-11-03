fn main(){
    let mut score:Vec<i32> = Vec::new();

    (&mut score).push(85);
    (&mut score).push(90);

    for i in &score{
        println!("score is {}", i)

    }
    println!("first score: {}", score[0]);

   let owned = String::from("hello");
   let borrowed  = "world";

   println!("{}, {}", owned, borrowed);
}