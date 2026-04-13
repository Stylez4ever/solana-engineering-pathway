use std::collections::HashSet;

#[derive(Debug)]
struct Playlist {
    songs: Vec<String>,
    users: HashSet<String>,
}

impl FromIterator<(String, String)> for Playlist {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut songs = Vec::new();
        let mut users = HashSet::new();
        for (song, user) in iter {
            songs.push(song);
            users.insert(user);
        }
        Self { songs, users}
    }
}


fn main() {
    //let fifty_numbers = 1..=50;

    //let results = Vec::from_iter(fifty_numbers.clone());
    //println!("{results:?}");

    //let results = fifty_numbers.clone().collect::<Vec<i32>>();
    //println!("{:?}", results);

    // the goal is to remove diplicates
    //let unique_set: HashSet<_> = HashSet::from_iter(fifty_numbers.clone());
    //println!("{:?}", unique_set);

    //let unique_set = fifty_numbers.clone().collect::<HashSet<i32>>();
    //println!("{:?}", unique_set);
    
    //let chars = ['H', 'e', 'l', 'l', 'o'];
    //let greeting = String::from_iter(chars);
    //println!("{greeting}");

    let songs = [
        (String::from("Namela thaba"), String::from("Thabang")),
        (String::from("Mapedi nameng"), String::from("Thabang")),
        (String::from("Bare monna ke se tunya"), String::from("Khomotxo")),
        (String::from("Di tau txa bolaya"), String::from("Bafana")),
    ];

    //let playlist: Playlist = Playlist::from_iter(songs);
    //println!("{playlist:?}");

    let playlist: Playlist = songs.into_iter().collect::<Playlist>();
    println!("{playlist:?}");



}





