use bimap::BiMap;

pub struct BFCode {
    pub loops: BiMap<usize, usize>,
    pub text: String
}

impl BFCode {
    pub fn new(text: &String) -> BFCode {
        BFCode {
            loops: BiMap::new(),
            text: text.clone()
        }
    }
}
