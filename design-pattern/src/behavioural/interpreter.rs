pub struct Interpreter<'a> {
    it: std::str::Chars<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(infix: &'a str) -> Self {
        Self { it: infix.chars() }
    }

    fn next_char(&mut self) -> Option<char> {
        self.it.next()
    }

    pub fn interpret(&mut self, out: &mut String) {
        self.term(out);

        while let Some(op) = self.next_char() {
            if op == '+' || op == '-' {
                self.term(out);
                out.push(op);
            } else {
                panic!("Unexpected symbol '{}'", op);
            }
        }
    }

    pub fn term(&mut self, out: &mut String) {
        match self.next_char() {
            Some(ch) if ch.is_digit(10) => out.push(ch),
            Some(ch) => panic!("Unexpected symbol '{}'", ch),
            None => panic!("Unexpected end of string"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter() {
        let mut itr = Interpreter::new("2+3");
        let mut postfix = String::new();
        itr.interpret(&mut postfix);
        assert_eq!(postfix, "23+");

        itr = Interpreter::new("1-2+3-4");
        postfix.clear();
        itr.interpret(&mut postfix);
        assert_eq!(postfix, "12-3+4-");
    }

    #[should_panic]
    #[test]
    fn test_wrong_interpreter() {
        let mut itr = Interpreter::new("2+4+");
        let mut postfix = String::new();
        itr.interpret(&mut postfix);
    }
}
