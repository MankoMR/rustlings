// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)


mod sausage_factory {
    pub fn make_sausage() {
        println!("sausage!");
    }
    pub struct Test {
        pub public: String,
        private: String,
    }
    impl Test{
        pub fn new(content: String) -> Test{
            Test{public: content,private: String::from("Secreeet")}
        }
        pub fn get_private(&self) -> &str {
            &self.private
        }
    }
}

fn main() {
    sausage_factory::make_sausage();

    let _test = sausage_factory::Test::new(String::from("Alloa"));
    //let _test_without_new = sausage_factory::Test{public: String::from("sfsd")};
    let info = _test.get_private();
    println!("content is {}",info);
}
