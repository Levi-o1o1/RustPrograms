// Traits 
// if the all methods share same behaviour eg we have dog cat cow all have eat(), 
// instead of writing separate rules for every animal, we use trait

struct Dog;
struct Cat;

trait Animal{  // trait is a keyword
    fn make_sound();

}

// Implementing trait for Dog
impl Animal for Dog{
    fn make_sound(){
        println!("bork ");
    }
}
// Implementing trait for cat
impl Animal for Cat{
    fn make_sound(){
        println!("meow meow");
    }
}

fn main(){
    Dog::make_sound();
    Cat::make_sound();
}




