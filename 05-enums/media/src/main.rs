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
    let good_movie = Media::Movie {
        title: String::from("The Dark Knight"),
        director: String::from("Christopher Nolan"),
    };
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };

    print_media(audiobook);
    print_media(good_movie);
    print_media(bad_book);
}
