pub trait PadStr {
    fn pad_start(&self, width: usize, pad_char: char) -> String;
}

impl PadStr for str {
    fn pad_start(&self, width: usize, pad_char: char) -> String {
        let words = self.len();

        if words < width {
            let diff = width - words;
            let mut s = String::new();

            for _ in 0..diff {
                s.push(pad_char)
            }

            s.push_str(self);

            return s;
        }

        return self.to_string();
    }
}
