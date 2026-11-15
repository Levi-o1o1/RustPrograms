fn main(){


    // for loop

    let mut cout : i32 =0;

    let result :i32 = loop{
        cout += 1;
        if cout == 5 {
            break cout *  2
        }
    };

    println!("loop result is : {}", result);

    // while loop 

    let mut n: i32 = 5;

    while n!= 0{
        println!("{}",n);
        n-=1;
    }
    println!("hello");


    //range loop

    for i in 1..5 {
        println!("{i}");
    }

}