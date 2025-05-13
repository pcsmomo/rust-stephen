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

fn print_beamline(beamline: &Beamline) {
    println!("{:#?}", beamline);
}

fn reset_pub(beamline: &mut Beamline) {
    beamline.publications = 0;
}

fn main() {
    let beamline = Beamline::new(10, String::from("Beamline 1"));
    // print_beamline(beamline);
    // print_beamline(beamline);

    // let list_of_bls = vec![beamline];
    // println!("{:#?}", beamline);
    print_beamline(&beamline);
    print_beamline(&beamline);

    // let mut beamline2 = Beamline::new(10, String::from("Beamline 2"));
    // reset_pub(&mut beamline2);
    // println!("{:#?}", beamline2);
}
