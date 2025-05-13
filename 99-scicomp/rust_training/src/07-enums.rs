#[derive(Debug)]
enum Detector {
    Eiger {
        frame_time: f32,
        temperature: f32,
    },
    PCOEdge {
        magnification: f32,
        acquire_time: f32,
    },
    Blackfly {
        acquire_time: f32,
    },
}

impl Detector {
    fn description(&self) -> String {
        match self {
            Detector::Eiger {
                frame_time,
                temperature,
            } => {
                format!("Eiger: {} {}", frame_time, temperature)
            }
            Detector::PCOEdge {
                magnification,
                acquire_time,
            } => {
                format!("PCOEdge: {} {}", magnification, acquire_time)
            }
            Detector::Blackfly { acquire_time } => {
                format!("Blackfly: {}", acquire_time)
            }
            _ => {
                format!("Unknown detector")
            }
        }
    }
}

fn print_detector(detector: &Detector) {
    println!("{:#?}", detector);
}

fn main() {
    let eiger = Detector::Eiger {
        frame_time: 0.1,
        temperature: 20.0,
    };

    print_detector(&eiger);
    println!(
        "{}",
        if let Detector::Eiger { frame_time, .. } = eiger {
            frame_time
        } else {
            0.0
        }
    );

    println!("{}", eiger.description());
}
