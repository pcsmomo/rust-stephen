#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audio { title: String },
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::Audio {
        title: String::from("The Great Gatsby"),
        // title: "The Great Gatsby".to_string(),
    };

    print_media(audiobook);
}
