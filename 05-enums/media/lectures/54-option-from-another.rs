#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audio { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        // if let Media::Book { title, author } = self {
        //     format!("Book: {} by {}", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("Movie: {} by {}", title, director)
        // } else if let Media::Audio { title } = self {
        //     format!("Audiobook: {}", title)
        // } else {
        //     String::from("Unknown media")
        // }

        // Using `match` keyword
        match self {
            Media::Book { title, author } => {
                format!("Book: {} by {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} by {}", title, director)
            }
            Media::Audio { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcast(episode_number) => {
                format!("Podcast episode: {}", episode_number)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            // Good! We have somethingg to return
            Some(&self.items[index])
        } else {
            // Bad! We don't have anything to return!!
            None
        }
    }
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
    let podcast = Media::Podcast(7);
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    // let item = catalog.get_by_index(40);

    // println!("{:#?}", item)

    // Using `match`
    match catalog.get_by_index(0) {
        Some(media) => {
            println!("{:#?}", media);
        }
        None => {
            println!("No value available");
        }
    }

    // Using `if let`
    // if let MightHaveAValue::ThereIsAValue(media) = catalog.get_by_index(99) {
    //     println!("{:#?}", media);
    // } else {
    //     println!("No value available");
    // }
}
