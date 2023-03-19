const BASE_FREQ: f32 = 440.0;

#[derive(Debug, PartialEq, Eq)]
pub enum Note {
    A = 0,
    AS = 1,
    B = 2,
    C = 3,
    CS = 4,
    D = 5,
    DS = 6,
    E = 7,
    F = 8,
    FS = 9,
    G = 10,
    GS = 11,
}

impl ToString for Note {
    fn to_string(&self) -> String {
        match self {
            &Note::A => "A",
            &Note::AS => "A#",
            &Note::B => "B",
            &Note::C => "C",
            &Note::CS => "C#",
            &Note::D => "D",
            &Note::DS => "D#",
            &Note::E => "E",
            &Note::F => "F",
            &Note::FS => "F#",
            &Note::G => "G",
            &Note::GS => "G#",
        }
        .to_string()
    }
}

impl TryFrom<u8> for Note {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Note::*;
        match value {
            0 => Ok(A),
            1 => Ok(AS),
            2 => Ok(B),
            3 => Ok(C),
            4 => Ok(CS),
            5 => Ok(D),
            6 => Ok(DS),
            7 => Ok(E),
            8 => Ok(F),
            9 => Ok(FS),
            10 => Ok(G),
            11 => Ok(GS),
            _ => Err(()),
        }
    }
}

pub fn find_closest_note(pitch: f32) -> (Note, f32) {
    let note_index = ((pitch / BASE_FREQ).log2() * 12.0).round().rem_euclid(12.0) as u8;
    let closest_note: Note = note_index.try_into().unwrap();
    let closest_pitch = (BASE_FREQ * 2.0).powf(note_index as f32 / 12.0);

    (closest_note, closest_pitch)
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_closest_note() {
        use super::*;

        assert_eq!(find_closest_note(82.41).0, Note::E);
        assert_eq!(find_closest_note(110.0).0, Note::A);
        assert_eq!(find_closest_note(146.83).0, Note::D);
        assert_eq!(find_closest_note(196.0).0, Note::G);
        assert_eq!(find_closest_note(246.94).0, Note::B);
        assert_eq!(find_closest_note(329.63).0, Note::E);
    }
}
