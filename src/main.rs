fn cetak_deskripsi(item: &impl Deskripsi) {
  println!("{}", item.deskripsi());
}

struct Buku {
  judul: String,
  penulis: String,
}

struct Kelas {
  judul: String,
  penulis: String,
}

trait Deskripsi {
  fn deskripsi(&self) -> String;
}


struct Film {
  judul: String,
  sutradara: String,
}

impl Deskripsi for Buku {
  fn deskripsi(&self) -> String {
      format!("Buku: '{}', ditulis oleh {}", self.judul, self.penulis)
  }
}

impl Deskripsi for Film {
  fn deskripsi(&self) -> String {
      format!("Film: '{}', disutradarai oleh {}", self.judul, self.sutradara)
  }
}


fn main() {
  let buku = Buku {
      judul: String::from("Pemrograman Rust"),
      penulis: String::from("John Doe"),
  };

  let film = Film {
      judul: String::from("Rust: The Movie"),
      sutradara: String::from("Jane Smith"),
  };

  cetak_deskripsi(&buku);
  cetak_deskripsi(&film);
}
