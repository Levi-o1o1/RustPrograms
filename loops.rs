fn main(){


    // loops

    let mut cout : i32 =0;

    let result :i32 = loop{
        cout += 1;
        if cout == 5 {
            break cout *  2
        }
    };

    println!("loop result is : {}", result)

}