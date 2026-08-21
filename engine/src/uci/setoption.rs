pub fn parse<'a>(stream: &mut impl Iterator<Item = &'a str>, mut func: impl FnMut(&str, &str)) {
    if stream.next() != Some("name") {
        return;
    }

    let name = stream.next();

    if stream.next() != Some("value") {
        return;
    }

    let value = stream.next();

    if let (Some(name), Some(value)) = (name, value) {
        func(name, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_str(input: &str) -> Option<(String, String)> {
        let mut stream = input.split_whitespace();
        let mut result = None;
        parse(&mut stream, |name, value| {
            result = Some((name.to_owned(), value.to_owned()));
        });
        result
    }

    #[test]
    fn name_value_pair_is_parsed() {
        assert_eq!(
            parse_str("name Hash value 1000"),
            Some(("Hash".to_owned(), "1000".to_owned()))
        );
    }

    #[test]
    fn missing_name_keyword_is_ignored() {
        assert_eq!(parse_str("Hash value 1000"), None);
    }

    #[test]
    fn missing_value_keyword_is_ignored() {
        assert_eq!(parse_str("name Hash 1000"), None);
    }

    #[test]
    fn missing_value_is_ignored() {
        assert_eq!(parse_str("name Hash value"), None);
    }

    #[test]
    fn empty_input_is_ignored() {
        assert_eq!(parse_str(""), None);
    }
}
