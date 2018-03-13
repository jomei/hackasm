pub struct Line {
    inner: String
}

impl Line {
    const A_MARKER: char = '@';
    const C_MARKER: char = '(';

    pub fn new(str: String) -> Self {
        Line{ inner: str }
    }

    pub fn is_a(&self) -> bool {
        return self.inner.starts_with(Line::A_MARKER)
    }

    pub fn is_c(&self) -> bool {
        return self.inner.starts_with(Line::C_MARKER)
    }

    pub fn is_symbol(&self) -> bool {
        return self.is_a() || self.is_c()
    }

    pub fn symbol(&self) -> Option<String> {
        if !self.is_symbol() {
            return None
        }

        if self.is_a() {
            return Some(self.a_symbol())
        } else {
            return Some(self.c_symbol())
        }
    }

    fn a_symbol(&self) -> String {
        return self.inner.chars().skip(1).collect()
    }

    fn c_symbol(&self) -> String {
        return self.inner
            .split(|c| c == Line::C_MARKER || c == ')')
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_a() -> Line {
        return Line::new("@R1".to_string())
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
        return Line::new("(LOOP)".to_string());
    }

    #[test]
    fn is_c() {
        assert!(get_c().is_c());
    }

    #[test]
    fn c_symbol() {
        assert_eq!(get_c().c_symbol(), "LOOP");
    }

    #[test]
    fn is_symbol() {
        assert!(get_a().is_symbol());
        assert!(get_c().is_symbol());
    }
}