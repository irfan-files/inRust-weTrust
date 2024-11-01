fn main() {
    // Character - satu karakter
    let initial: char = 'R';
    println!("Inisial: {}", initial);

    // String Literal - kumpulan karakter
    let language: &str = "Rust Programming";
    println!("Bahasa: {}", language);

    // String Owned - bisa dimodifikasi
    let mut greeting = String::from("Hello");
    greeting.push(' '); // Tambah spasi
    greeting.push_str("Rust!");
    println!("Salam: {}", greeting);

    // Konversi antara tipe
    let first_char = language.chars().next().unwrap(); // Ambil karakter pertama
    println!("Karakter pertama: {}", first_char);
}
