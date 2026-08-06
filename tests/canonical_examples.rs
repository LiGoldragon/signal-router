use dotos::{DotosEncode, DotosSource};
use signal_router::{z2VXoV, z2VZGC};

#[test]
fn canonical_dotos_examples_are_exact_root_witnesses() {
    let examples = include_str!("../examples/canonical.dotos");
    let values = examples
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with(";;"))
        .collect::<Vec<_>>();

    assert_eq!(values.len(), 24);
    for text in &values[..9] {
        let value = DotosSource::new(text)
            .parse::<z2VZGC>()
            .expect("canonical request decodes");
        assert_eq!(value.to_dotos(), *text);
    }
    for text in &values[9..] {
        let value = DotosSource::new(text)
            .parse::<z2VXoV>()
            .expect("canonical reply decodes");
        assert_eq!(value.to_dotos(), *text);
    }
}
