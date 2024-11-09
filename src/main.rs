use rand::Rng;

#[allow(unused_variables)]
#[allow(unused_assignments)]


fn main() {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0,11);
    if num >= 5 {
        println!("{} is even", num);
    } else {
        println!("{} is odd", num);
    }
}
