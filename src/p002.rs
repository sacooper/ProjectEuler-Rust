fn main(){
    println!("Euler problem #2\nProblem:");
    println!("\tBy considering the terms in the Fibonacci sequence whose values do")
    println!("\tnot exceed four million, find the sum of the even-valued terms.");
    println!("Solution: \n\t{}", even_fib_sum(1, 1))
}

fn even_fib_sum(a : i32, b : i32) -> i32 {
    if a + b > 4_000_000 {
        0
    } else {
        let b2 = a + b;
        even_fib_sum (b, b2) + if b2 % 2 == 0 {b2} else {0}
    }
}
