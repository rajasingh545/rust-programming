enum Musician {
    SingerSongWriter(String),
    Band(u32),
}

use Musician::{Band, SingerSongWriter};

impl PartialEq for Musician {
    fn eq(&self, other: &Self) -> bool {
        match self {
            SingerSongWriter(name) => match other {
                SingerSongWriter(other_name) => name == other_name,
                Band(_) => false,
            },
            Band(size) => match other {
                Band(other_size) => size == other_size,
                SingerSongWriter(_) => false,
            },
        }
    }
}

fn main() {
    let raja_bieber = SingerSongWriter(String::from("Raja bieber"));
    let raja_dragon = SingerSongWriter(String::from("Raja bieber"));
    let holly = SingerSongWriter(String::from("Holly"));

    let rust_no_one = Band(0);
    let rust_worthy = Band(5);
    let rust_vengeance = Band(4);

    println!("{}", raja_bieber == raja_dragon);
    println!("{}", rust_no_one == rust_vengeance);

    print!("{}", holly == rust_worthy);
}
