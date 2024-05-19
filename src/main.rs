fn fibonacci(x: i32) {
    let mut a = 0;
    let mut b = 1;
    let mut f = 0;
    
    while a <= x {
        print!("{a}, ");
        f = a + b;
        a = b;
        b = f;
    }
}

fn main() {
    fibonacci(10);
}
