fn apply_function<F>(value: i32, func: F) -> i32
where
    F: Fn(i32) -> i32,
{
    func(value)
}

fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    let result = apply_function(5, square);
    println!("Hasil: {}", result); // Output: Hasil: 25
}
