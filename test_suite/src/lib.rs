mod json_validator;

#[cfg(test)]
mod tests {
    use insta::assert_snapshot_matches;
    use jens::{Block, File};
    use jens_derive::Jens;

    #[derive(Jens)]
    #[template = "test.jens"]
    struct Xyz {}

    #[test]
    fn test_derive_simple() {
        let t1 = Xyz::template1();
        let t2 = Xyz::template2(t1, "Hullo");

        assert_snapshot_matches!("test_derive_simple", t2.to_string());
    }

}
