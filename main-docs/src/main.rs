fn main() {
    for i in 0..12 {
        println!("Fib {}: {}", i, fibonacci(i));
    }
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return 1
    }
    let mut fibonacci_numbers: Vec<i32> = vec![0; (n+1) as usize];
    fibonacci_numbers[0] = 1;
    fibonacci_numbers[1] = 1;
    for index in 2..fibonacci_numbers.len() {
        fibonacci_numbers[index] = fibonacci_numbers[index - 1] + fibonacci_numbers[index - 2];
    }
    return fibonacci_numbers[n as usize]
}
