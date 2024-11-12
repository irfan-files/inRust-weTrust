

#[allow(unused_variables)]
#[allow(unused_assignments)]


fn main() {
   let a = |a:i32| a+1;
println!("{}",a(5));
let b = |b:i32| -> i32 {
  let c = b+1;
    c
};
println!("{}",b(5));

    }
  
