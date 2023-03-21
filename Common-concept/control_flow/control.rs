// *    CONTROL FLOWS
// * if , else , if-else

fn main  () {

    let mut age = 24;

    //* if
    if age > 18 {
        println!("Vote");
    }

    age = 12;

    //* if else 
    if age > 18 {
        println!("Vote");
    } else {
        println!(" No Vote");
    }

    // **  if else-if
    let num = 18;

    if num % 2 == 0 {
        println!("Divisible by 2");
    }  else if  num % 3 == 0 {
        println!("Divisible by 3");
    }  else if  num % 4 == 0 {
        println!("Divisible by 4");  
    }

}