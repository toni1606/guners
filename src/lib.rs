pub enum Notes {
    A(bool),
    B,
    C(bool),
    D(bool),
    E,
    F(bool),
    G(bool),
}

impl ToString for Notes {
    fn to_string(&self) -> String {
        match self {
            &Notes::A(false) => "A",
            &Notes::A(true) => "A#",
            &Notes::B => "B",
            &Notes::C(false) => "C",
            &Notes::C(true) => "C#",
            &Notes::D(false) => "D",
            &Notes::D(true) => "D#",
            &Notes::E => "E",
            &Notes::F(false) => "F",
            &Notes::F(true) => "F#",
            &Notes::G(false) => "G",
            &Notes::G(true) => "G#",
        }
        .to_string()
    }
}
