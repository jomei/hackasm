pub struct Line {
    pub inner: String,
    pub number: usize
}

impl Line {
    const A_MARKER: char = '@';
    const LABEL_MARKER: char = '(';

    pub fn new(str: String, line_number: usize) -> Self {
        Line{ inner: str, number: line_number }
    }

    pub fn is_a(&self) -> bool {
        self.inner.starts_with(Line::A_MARKER)
    }

    pub fn is_label(&self) -> bool {
        self.inner.starts_with(Line::LABEL_MARKER)
    }

    pub fn is_symbol(&self) -> bool {
        self.is_a() || self.is_label()
    }

    pub fn symbol(&self) -> Option<String> {
        if !self.is_symbol() {
            return None
        }

        if self.is_a() {
            Some(self.a_symbol())
        } else {
            Some(self.label_symbol())
        }
    }

    pub fn get_jump(&self) -> String {
        let split: Vec<&str> = self.inner.split(";").collect();
        if split.len() > 1 {
            split[1].to_string()
        } else {
            "".to_string()
        }
    }

    pub fn get_comp(&self) -> String {
        let split: Vec<&str> = self.inner.split(";").collect();
        let dest_comp: Vec<&str> = split[0].split("=").collect();
        if dest_comp.len() > 1 {
            dest_comp[1].to_string()
        } else {
            "".to_string()
        }
    }

    pub fn get_dest(&self) -> String {
        let split: Vec<&str> = self.inner.split(";").collect();
        let dest_comp: Vec<&str> = split[0].split("=").collect();
        dest_comp[0].to_string()
    }

    fn a_symbol(&self) -> String {
        self.inner.chars().skip(1).collect()
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
        Line::new("@some_var".to_string(), 0)
    }

    #[test]
    fn is_a() {
        assert!(get_a().is_a());
    }

    #[test]
    fn a_symbol() {
        assert_eq!(get_a().a_symbol(), "some_var");
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

    #[test]
    fn get_comp() {
        let line =  Line::new("D=A".to_string(), 0);
        assert_eq!("A", line.get_comp())
    }

    #[test]
    fn get_dest() {
        let line =  Line::new("D=A".to_string(), 0);
        assert_eq!("D", line.get_dest())
    }
}