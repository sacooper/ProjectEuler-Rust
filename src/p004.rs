fn main(){
    println!("Euler Problem #4");
    println!("Problem\tFind the largest palindrome made from the product of two 3-digit numbers");
    println!("Solution: {}", largest_palendrome(100, 999));
}

fn largest_palendrome(min : i32, max : i32) -> i32 {
    let mut solution = 0;
    for a in std::iter::range_step_inclusive(max, min, -1){
        for b in std::iter::range_step_inclusive(max, a, -1){
            let c = a*b;
            if c <= solution {break}
            if is_palendrome(c) {solution = c}
        }
    }

    return solution;
}

fn is_palendrome(a : i32) -> bool{
    let mut y = 0i32;
    let mut x = a;
    while x > 0 {
        y = y*10 + x%10;
        x /= 10;
    }
    return a==y
}
