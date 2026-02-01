use std::io;

fn main() {
    println!("hello world");
    let mut x = 5;
    let y: u32 = 6;
    x = 7;
    let tup: (i32, bool, char) = (1, true, 's');
    let arr: [i32; 5]= [1, 2, 3, 4, 5];
    arr[3];
    {

    }
    
    const SECONDS_IN_MIN: u32 = (50);

    // getting input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    //let in_input: i64 = input.trim().parse().unwrap();

    println!("the answer was {}", input);


    let test = (127_000 as i64);





    let food = "cookie";
    if food == "cookie" {

    }











    println!("x is: {}", SECONDS_IN_MIN)
}
