fn main(){
    println!("Euler Problem #3");
    println!("Problem:\n\tWhat is the largest prime factor of the number 600851475143 ?");
    println!("Solution:\n\t{}", solution())
}

fn solution() -> i64 {
    let mut r = 1;
    for a in std::iter::range_inclusive(2,20i64){
        r = lcm(a, r);
    };
    r
}

fn lcm(a : i64, b: i64) -> i64{
    a*b/gcd(a,b)
}

fn gcd(a : i64, b: i64) -> i64{
    use std::cmp::min;
    for x in std::iter::range_step_inclusive(min(a,b), 1, -1){
        if (a % x == 0) && (b % x == 0){ return x}
    };
    return 1
}
