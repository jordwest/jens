
use jens_derive::Jens;

#[derive(Jens)]
#[template = "test.jens"]
struct Xyz {}

#[test]
fn test_derive() {
        Xyz::template1();
        Xyz::template2();
        println!("Works!");
}
