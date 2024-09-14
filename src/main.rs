fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x in the outer scope is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces); 
}
