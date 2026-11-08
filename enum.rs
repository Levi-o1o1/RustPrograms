enum TrafficLight{ // one an enum exactly one of these variants at a time
    Red,  // variants without any data
    Yellow,
    Green,


}


fn main(){

    let light = TrafficLight::Red;
   

    match light{
        TrafficLight::Red => println!("Stop"),
        TrafficLight::Yellow => println!("not now"),
        TrafficLight::Green => println!("Go go go ."),
    }
}