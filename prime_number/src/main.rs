
use std::time::Instant;

fn is_prime(n:i32) -> bool {

    if n==0 || n==1 {
        return false; // false 와 return false; 의 차이??
    }

    for i in 2..=(n/2) {

        if n % i == 0 {
            return false;
        }

    }

    true
}


fn main() {

    let start = Instant::now();
    let num = 100_000;
    let mut prime = 0;


    for i in 1..num {
        if is_prime(i){
            prime += 1;
        }
    }

    let duration = start.elapsed();
    println!("Primes: {prime}");
    println!("Rust Time {:.7}", duration.as_secs_f64());

}
