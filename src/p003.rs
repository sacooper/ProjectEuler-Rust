fn main(){
    let mut num = 600851475143i64;
    let mut i = 2;
    while num != 1{
        if num % i == 0 {
            num /= i
        } else { i += 1}
    }

    println!("Euler Problem #3");
    println!("Problem:\n\tWhat is the largest prime factor of the number 600851475143 ?")
    println!("Solution:\n\t{}", i)
}
