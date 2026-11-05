// enum with data (very imp)

// this is where rust enums become powerfull unlike a basic enum, rust allows each variant to hold its own data.

enum Payment{
    Cash,
    Card(String),
    UPI(String),
}

fn main(){
    let payment1 = Payment::Cash;  // no data 

    let payment2 = Payment::Card(  // store a  string data
        String::from("Visa")
    );
    let payment3 = Payment::UPI(  // data also here 
        String::from("Raj@upi")
    );

}