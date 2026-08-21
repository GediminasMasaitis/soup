use crate::search::SearchParameters;

pub fn parse<'a>(mut stream: impl Iterator<Item = &'a str>) -> SearchParameters {
    let mut params = SearchParameters::default();

    while let Some(word) = stream.next() {
        match word {
            "wtime" => params.wtime = stream.next().and_then(|w| w.parse().ok()),
            "btime" => params.btime = stream.next().and_then(|w| w.parse().ok()),
            "winc" => params.winc = stream.next().and_then(|w| w.parse().ok()),
            "binc" => params.binc = stream.next().and_then(|w| w.parse().ok()),
            "movetime" => params.movetime = stream.next().and_then(|w| w.parse().ok()),
            "depth" => params.depth = stream.next().and_then(|w| w.parse().ok()),
            "nodes" => params.nodes = stream.next().and_then(|w| w.parse().ok()),
            "infinite" => params.infinite = true,
            _ => {}
        }
    }

    params
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limits_are_parsed() {
        let params = parse(
            "wtime 1000 btime 2000 winc 10 binc 20 movetime 50 depth 3 nodes 400"
                .split_whitespace(),
        );
        assert_eq!(params.wtime, Some(1000));
        assert_eq!(params.btime, Some(2000));
        assert_eq!(params.winc, Some(10));
        assert_eq!(params.binc, Some(20));
        assert_eq!(params.movetime, Some(50));
        assert_eq!(params.depth, Some(3));
        assert_eq!(params.nodes, Some(400));
        assert!(!params.infinite);
    }

    #[test]
    fn infinite_is_parsed() {
        assert!(parse("infinite".split_whitespace()).infinite);
    }

    #[test]
    fn bad_values_are_ignored() {
        let params = parse("depth banana nodes".split_whitespace());
        assert_eq!(params.depth, None);
        assert_eq!(params.nodes, None);
    }
}
