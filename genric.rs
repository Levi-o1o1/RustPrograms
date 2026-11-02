// traits and generics 
// this is a program for largest number in a vector  its non-generic version
/*fn largest(list: &[i32]) -> i32{
    let mut largest = list[0];

    for &item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}

fn main(){

    let numbers = vec![10,203,44,3,8];
    println!("largest {}", largest(&numbers))
}
*/
// **************************************** TO RUN UPPER CODE PLEASE UNCOMMNET FIRST & HIDE DOWN CODE ******************************
// this is generic version 
fn largest <T: PartialOrd + Copy>(list: &[T]) -> T{    // we only change this line T is genric placeholder its find oder wise how is big*/;    // we onluy change isthis line T is genric place holder its ofind oder wise how is bigf*/;    // we onluy change isthis line T is genric place holder its ofind oder wise how is bigf*/;    // we onluy change isthis line T is genric place holder its ofind oder wise how is bigf*/;    // we onluy change isthis line T is genric place holder its ofind oder wise how is bigf*/;    // we onluy change isthis line T is genric place holder its ofind oder wise how is bigf*/;    // we onluy change isthis line T is genric place holder its ofind oder wise how is bigf*/;    // we onluy change isthis line T is genric place holder its ofind oder wise how is bigf
    let mut largest = list[0];

    for &item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}

fn main(){

    let numbers = vec![10,203,44,3,8];
    println!("largest {}", largest(&numbers));
    let chars = vec!['a','b','c','d','e'];
    println!("largest {}", largest(&chars))
}