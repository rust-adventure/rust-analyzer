mod mytrait;

#[derive(Debug)]
struct Thing;

fn main() {
    let thing = Thing;
    println!("Hello, {}", thing);
}

impl std::fmt::Display for Thing {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", "a thing")
    }
}
