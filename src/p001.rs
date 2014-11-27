fn main() {
    let mut sum = 0i64;

    let mut i = 0i;
    while i < 1000{
        if i % 3 == 0 || i % 5 == 0 {sum += i as i64}
        i += 1;
    }

    println!("Sum: {}", sum);
}
