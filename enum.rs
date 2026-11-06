// enum(enumeration) is a custom data type that allow you to define a set of possible
// variants(choices).
// think of it : one variable, multiple possible choices, but only one active choice at a time

enum TrafficLight{   // pascal case is not allow in rust only camel or snake case name only
    Red,   // make sure it uppercase
    Yellow,
    Green,
}

fn main(){

    let signal = TrafficLight::Green;

    println!("You can go !");

    println!("{:?}",instruction());
}


//enum with match (*imp)

enum Tlight{
    Red,
    Green,
    Yellow,
}

fn instruction(){
    let signal = Tlight::Green;

    match signal{
        Tlight::Red => {
            println!("Stop");
        }
        Tlight::Green => {
            println!("Go");
        }
        Tlight::Yellow => {
            println!("not now");
        }
    }
}