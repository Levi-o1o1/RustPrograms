fn main() {
      // Scalars data types


 println!("**************************************************************************************************************************** \n ");
      let age : u32 = 18; // non -negative 32bit integer
      let num : i32 = -34; // default for numbers if you dont specify type
      let temp  : f64 = 18.33; // 64 bit floating point number for decimal
      let is_on : bool = true; // true & false
      let grade : char = 'B';  // a single char

      println!("age is {}, num is {},temp is {}, bool is {}, grade is {}", age, num , temp, is_on, grade);


      // compound type


      // tuple

      let per : (&str, u32, u32) = ("Raj",23, 500000);
      println!("{} is {} years old , and his sallary is {}", per.0, per.1, per.2);


      // Array

      let score : [i32; 3] = [90,34,100];

      println!("first ele {} ,  second ele {} , third ele {} \n", score[0], score[1], score[2] );


      




 println!("**************************************************************************************************************************** \n ");
      //  aditional info about data type 
  // interger 
  // 8-bit  singed i8  unsigned u8   , range -128 to 127  value we store	
  // 16-bit  singed i16  unsigned u16, range -32768 to 32767 value we store	
  // 32-bit  singed i32  unsigned u32, range -2147483648 to 2147483647 value we store	
  // 64-bit  singed i64  unsigned u64, range -9223372036854775808 to 9223372036854775807 value we store	
  // 128-bit  singed i128  unsigned u128	
  let a: i8 = 127;
  println!("this is 8-bit int: {}", a);
  let b: i16 = 32767;
  println!("this is 16-bit int: {}", b);
  let c: i32 = 2147483647;
  println!("this is 32-bit int: {}", c);
  let d: i64 = 9223372036854775807;
  println!("this is 64-bit int: {}", d);


 println!("**************************************************************************************************************************** \n ");
      
}