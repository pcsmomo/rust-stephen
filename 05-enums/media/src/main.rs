mod content;

use content::catalog::Catalog;
use content::media::Media;

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
    // catalog.add(placeholder);

    let item = catalog.get_by_index(40);

    // println!("{:#?}", item.unwrap());
    // println!("{:#?}", item.expect("expected there to be an item here"));
    println!("{:#?}", item.unwrap_or(&placeholder));
}
