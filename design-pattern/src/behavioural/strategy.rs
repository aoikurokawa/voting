use std::collections::HashMap;

type Data = HashMap<String, u32>;

trait Formatter {
    fn format(&self, data: &Data, buf: &mut String);
}

struct Report;

impl Report {
    // Write should be used but we kept it as String to ignore error handling
    fn generate<T: Formatter>(g: T, s: &mut String) {
        // background operations...
        let mut data = HashMap::new();
        data.insert("one".to_string(), 1);
        data.insert("two".to_string(), 2);

        // generate report
        g.format(&data, s);
    }
}

struct Text;
impl Formatter for Text {
    fn format(&self, data: &Data, buf: &mut String) {
        for (k, v) in data {
            let entry = format!("{} {}\n", k, v);
            buf.push_str(&entry);
        }
    }
}

struct Json;
impl Formatter for Json {
    fn format(&self, data: &Data, buf: &mut String) {
        buf.push('[');
        for (k, v) in data.into_iter() {
            let entry = format!(r#"{{"{}": "{}"}}"#, k, v);
            buf.push_str(&entry);
            buf.push(',');
        }

        buf.pop(); // remove extra, at the end
        buf.push(']');
    }
}

struct Addr;
impl Addr {
    pub fn add<F>(x: u8, y: u8, f: F) -> u8
    where
        F: Fn(u8, u8) -> u8,
    {
        f(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_pattern() {
        let mut s = String::from("");
        Report::generate(Text, &mut s);
        assert!(s.contains("one 1"));
        assert!(s.contains("two 2"));

        s.clear();
        Report::generate(Json, &mut s);
        assert!(s.contains(r#"{"one": "1"}"#));
        assert!(s.contains(r#"{"two": "2"}"#));
    }

    #[test]
    fn test_strategy_pattern2() {
        let arith_addr = |x: u8, y: u8| x + y;
        let bool_addr = |x: u8, y: u8| {
            if x == 1 || y == 1 {
                1
            } else {
                0
            }
        };

        let custom_adder = |x: u8, y: u8| 2 * x + y;

        assert_eq!(9, Addr::add(4, 5, arith_addr));
        assert_eq!(0, Addr::add(0, 0, bool_addr));
        assert_eq!(5, Addr::add(1, 3, custom_adder));
    }
}
