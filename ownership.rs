fn main(){
    let a = String::from("Hello");
    let b = a; // a is now invalid

       println!("a  value :{}, b value {}", a, b);

    // in oder to solve problem use .clone() eg let b = a.clone();
    // Each value has one owner
}