fn main() {
    let angka = 7;
    if (angka & 1) == 0 {
        println!(" adalah bilangan genap");
    } else {
        println!(" adalah bilangan ganjil");
    }

    let x = 5;
    let hasil = x << 3;
    println!(" 5* 2 = {}", hasil);

    let y = 8;
    let hasil = y >> 3;
    println!(" 8/2 = {}", hasil);
}
