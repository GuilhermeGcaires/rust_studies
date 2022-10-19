use std::io;

fn main() {

    println!("Input N or to quit the program type `quit`");  
    
    loop {
        println!("Type a positive number");
        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("");

        if n.trim() == "quit" {
            break;
        }
        let n: u32 = match n.trim()
                            .parse() {
                            Ok(n) => n,
                            Err(_) => continue,
            };
        println!("Fibonacci ({}) => {}", n, fib(n));
    }
}

fn fib(n: u32) -> u32 {
    if n <= 0 {
       return 0;
    } else if n == 1 {
        return 1;
    } fib(n-1) + fib(n-2)
}
