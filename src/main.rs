use std::io::Read;

use bignum::bignum::BigNum;

fn main() {
    println!("BigNum demo\n===========");
    println!("BigNum can handle numbers up to e1.79e308. That's 10^(max of f64).");
    println!("The demonstration will start with 1.0 and print the value of 2^fib(n) until it overflows into infinity.\n");

    println!("Press Enter to start the demonstration.");
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();


    let mut cur = BigNum::from_f64(1.0);
    let mut prev = BigNum::from_f64(1.0);
    let mut iterations = 0;

    let two = BigNum::from_f64(2.0);

    let timer = std::time::Instant::now();
    loop {
        let next = cur + prev;
        prev = cur;
        cur = next;

        iterations += 1;
        
        let result = two.powb(cur);
        println!("Iteration {}: {}", iterations, result);

        if result.is_infinite() {
            println!("\nThe number has reached infinity after {} iterations.", iterations);
            break;
        }
    }
    let elapsed = timer.elapsed();
    println!("It took {} microseconds for the number to overflow.", elapsed.as_micros());

    println!("Press Enter to exit.");
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
}