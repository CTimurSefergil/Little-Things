// !!!!!! 97. SATIRDA EKRANA YAZILACAKLARI VERİYE BAĞLAMADIĞIM İÇİN ŞU AN VERİYE ULAŞAMIYORUM VE EKRANA MANUEL GİRMEM GEREKİYOR
// !!! BEĞENİLER SIFIRLANIYOR HER SEFERİNDE
// !!! CLONEDAN KURTULAMIYORUM
// !!! PROGRAM İÇİ PLAYLİST OLUŞTURAMADIM


use std::collections::HashMap;
use std::io::{stdin, stdout, BufReader, BufRead};
use std::fs::File;
use rodio::{Decoder, OutputStream, source::Source};
use std::path::Path;

// Müzikleri mp3 olarak ekle

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Music {
    name: String,
    likes: i32,
    dislikes: i32,
    path: String,
}

impl Music {
    fn add_music(name: String, mut likes: i32, mut dislikes: i32, path: String) -> Self {
        Self {
            name,
            likes,
            dislikes,
            path,
        }
    }

    fn like(&mut self) {
        self.likes = self.likes + 1
    }

    fn dislike(&mut self) {
        self.dislikes = self.dislikes + 1
    }
}

fn create_playlist() -> HashMap<String, String> {
    HashMap::new()
}

// ÇÖZDÜM LAN SENİ :DDDDDDDDDDDDDDDDDDDDDDDDDd
fn add_music_to_the_playlist(playlist: &mut HashMap<String, String>, music: Music) {
    if playlist.contains_key(&music.name) {
        println!("Already Exists In Playlist");
    } else {
        playlist.insert(music.name, music.path);
    }
}

fn read_voice_data(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}

fn main() {
    /*let musics = read_voice_data("music.csv");
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let song1 = BufReader::new(File::open(&musics[0]).unwrap());
    let source1 = Decoder::new(song1).unwrap();

    let song2 = BufReader::new(File::open(&musics[1]).unwrap());
    let source2 = Decoder::new(song2).unwrap();

    let song3 = BufReader::new(File::open(&musics[2]).unwrap());
    let source3 = Decoder::new(song3).unwrap();*/

    let mut music1 = Music::add_music("HeyJude".to_string(), 0, 0, "a".to_string());

    let mut music2 = Music::add_music("AçKapıyı".to_string(), 0, 0, "b".to_string());

    let mut music3 = Music::add_music("FranksChoice".to_string(), 0, 0, "c".to_string());

    let mut liste1 = create_playlist();
    let mut liste2 = create_playlist();

    add_music_to_the_playlist(&mut liste1, music1.clone());
    add_music_to_the_playlist(&mut liste1, music2.clone());
    add_music_to_the_playlist(&mut liste2, music3.clone());

    let mut planswer = String::new();
    println!("Which playlist do you want to enter 1 or 2");
    println!("{} and {}", stringify!(liste1), stringify!(liste2));
    stdin()
        .read_line(&mut planswer)
        .expect("Failed to read the answer");

    println!("{}", planswer);

    if planswer.trim() == String::from("liste1") {
        for music in liste1.keys() {
            println!("name of the song is {:?}, likes and dislikes are {:?}, {:?}", music, music1.likes, music1.dislikes);
        }

        let mut sanswer = String::new();
        println!("Which song do you want to listen");
        stdin()
            .read_line(&mut sanswer)
            .expect("Failed to read the answer");

        if liste1.contains_key(sanswer.trim()) {
            println!("{:?}", sanswer.trim());
        } else {
            println!("No such music found");
        }
    } else if planswer.trim() == String::from("liste2") {
        for music in liste2.keys() {
            println!("{:?}", music);
        }

        let mut sanswer = String::new();
        println!("Which song do you want to listen");
        stdin()
            .read_line(&mut sanswer)
            .expect("Failed to read the answer");

        if liste2.contains_key(sanswer.trim()) {
            println!("{:?}", sanswer.trim());
        } else {
            println!("No such music found");
        }
    }
}
