#[derive(Debug)]
struct Beamline {
    publications: u16,
    name: String,
}

impl Beamline {
    fn new(publications: u16, name: String) -> Self {
        Beamline { publications, name }
    }

    fn increase_publications(&mut self) {
        self.publications += 1;
    }
}

fn main() {
    let mut b1 = Beamline::new(10, String::from("Beamline 1"));

    b1.increase_publications();

    println!("Beamline 1 {:#?}", b1);
}
