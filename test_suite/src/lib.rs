mod json_validator;

#[cfg(test)]
mod tests {
    use insta::assert_snapshot_matches;
    use jens::Block;
    use jens_derive::Template;

    #[derive(Template)]
    #[filename = "test.jens"]
    struct Xyz {}

    #[test]
    fn test_derive_simple() {
        let t1 = Xyz::template1();
        let t2 = Xyz::template2(t1, "Hullo");

        assert_snapshot_matches!("test_derive_simple", t2.to_string());
    }

}
