#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
  let square = |a| a*a;
    apply(square, 6);

    let limit = 500;
    let mut sum = 0;
    for i in 0 .. {
      let isq = i*i;
      if isq > limit {break;}
      else {
        if is_even(isq) {
        sum += isq;
      }
    }
    }

    println!("loop sum = {}",sum); 
}

fn is_even(n: u32) -> bool {
  n % 2 == 0
} 

fn apply (f : fn(i32)-> i32, a:i32) {
  println!("result  {}",f(a));


}