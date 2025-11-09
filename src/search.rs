pub struct SearchInfo {
    depth: i8,
    time: u8,
}

impl SearchInfo {
    pub fn new() -> Self {
        SearchInfo { depth: -1, time: 0 }
    }
}
