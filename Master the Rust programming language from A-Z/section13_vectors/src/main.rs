#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Folder {
        Folder {
            name,
            contents: vec![],
        }
    }

    fn create_file(self: &mut Folder, name: String) {
        let file = File { name };

        self.contents.push(file);
    }

    fn delete_file(self: &mut Folder, index: usize) -> File {
        self.contents.remove(index)
    }

    fn get_file(self: &Folder, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}

fn main() {
    let mut folder = Folder::new(String::from("Docs"));

    folder.create_file(String::from("main.rs"));
    folder.create_file(String::from("lib.rs"));

    println!("{:#?}", folder);

    folder.delete_file(0);
    println!("{:#?}", folder);

    let match_folder = folder.get_file(1);
    match match_folder {
        Option::Some(index) => println!("{:?}", index),
        Option::None => println!("There was no file"),
    }
    
}


