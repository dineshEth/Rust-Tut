use std::io;

fn  main() {
    let a = [10,20,30,40,50];

    println!("Please enter an array index");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Fail to read line");
    
    let index: usize = index.trim().parse().expect("Fail to read");

    println!("The at index: {index} is : {}",a[index]);
}