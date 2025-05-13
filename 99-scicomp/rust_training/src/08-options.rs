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
                format!(
                    "Eiger - frame_time: {} temperature: {}",
                    frame_time, temperature
                )
            }
            Detector::PCOEdge {
                magnification,
                acquire_time,
            } => {
                format!(
                    "PCOEdge - magnification: {} acquire_time: {}",
                    magnification, acquire_time
                )
            }
            Detector::Blackfly { acquire_time } => {
                format!("Blackfly - acquire_time: {}", acquire_time)
            }
            _ => {
                format!("Unknown detector")
            }
        }
    }
}

struct DetectorPool {
    detectors: Vec<Detector>,
}

impl DetectorPool {
    fn new() -> Self {
        DetectorPool {
            detectors: Vec::new(),
        }
    }

    fn add(&mut self, detector: Detector) {
        self.detectors.push(detector);
    }

    fn get_by_index(&self, index: usize) -> Option<&Detector> {
        if index < self.detectors.len() {
            Some(&self.detectors[index])
        } else {
            None
        }
    }
}

fn main() {
    let mut detector_pool = DetectorPool::new();

    detector_pool.add(Detector::Eiger {
        frame_time: 0.1,
        temperature: 20.0,
    });

    detector_pool.add(Detector::PCOEdge {
        magnification: 1.0,
        acquire_time: 0.1,
    });

    detector_pool.add(Detector::Blackfly { acquire_time: 0.1 });

    // for detector in detector_pool.detectors {
    //     println!("{}", detector.description());
    // }

    if let Some(detector) = detector_pool.get_by_index(1) {
        println!("{}", detector.description());
    } else {
        println!("Detector not found");
    }
}
