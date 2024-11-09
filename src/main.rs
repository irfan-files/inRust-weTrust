

#[allow(unused_variables)]
#[allow(unused_assignments)]


fn main() {
    for i  in 1..=11{
      println!("{0} * {0} = {1}",i,i*i);
    }

    let pets = ["cat","dog","fish","bird","snake"];
    for pet in pets.iter(){

      if pet == &"snake"{
        println!("{}, I don't like snakes", pet);
        continue;
      }
      if pet == &"fish"{
        println!("{}, I it's a pet", pet);
        continue;
          
      }

        println!("{pet}");
    }

    for (pos,i) in (1..11).enumerate(){
      let square = i*i;
      let nb = pos+1;
      println!("{0} * {0} = {1}", nb,square);
    }
  }
