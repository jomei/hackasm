pub struct Line {
    inner: String,
    pub number: usize
}

impl Line {
    const A_MARKER: char = '@';
    const LABEL_MARKER: char = '(';

    pub fn new(str: String, line_number: usize) -> Self {
        Line{ inner: str, number: line_number }
    }

    pub fn is_a(&self) -> bool {
        return self.inner.starts_with(Line::A_MARKER) &&
            self.inner.to_uppercase() != self.inner // TODO: check if string is lowcase, should be simplified
    }

    pub fn is_label(&self) -> bool {
        return self.inner.starts_with(Line::LABEL_MARKER)
    }

    pub fn is_symbol(&self) -> bool {
        return self.is_a() || self.is_label()
    }

    pub fn symbol(&self) -> Option<String> {
        if !self.is_symbol() {
            return None
        }

        if self.is_a() {
            return Some(self.a_symbol())
        } else {
            return Some(self.label_symbol())
        }
    }

    fn a_symbol(&self) -> String {
        return self.inner.chars().skip(1).collect()
    }

    fn label_symbol(&self) -> String {
        return self.inner
            .split(|c| c == Line::LABEL_MARKER || c == ')')
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_a() -> Line {
        return Line::new("@R1".to_string(), 0)
    }

    #[test]
    fn is_a() {
        assert!(get_a().is_a());
    }

    #[test]
    fn a_symbol() {
        assert_eq!(get_a().a_symbol(), "R1");
    }

    fn get_c() -> Line {
        return Line::new("(LOOP)".to_string(), 0);
    }

    #[test]
    fn is_label() {
        assert!(get_c().is_label());
    }

    #[test]
    fn label_symbol() {
        assert_eq!(get_c().label_symbol(), "LOOP");
    }

    #[test]
    fn is_symbol() {
        assert!(get_a().is_symbol());
        assert!(get_c().is_symbol());
    }
}