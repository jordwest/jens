
use jens_derive::Jens;

#[derive(Jens)]
#[template = "test.jens"]
struct Xyz {}

#[test]
fn test_derive() {
        let t1 = Xyz::template1();
        println!("Template output:\n{}", Xyz::template2(t1, "Hullo"));
        println!("Works!");
}
