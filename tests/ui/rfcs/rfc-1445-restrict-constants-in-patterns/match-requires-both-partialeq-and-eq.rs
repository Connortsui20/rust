#[derive(Eq)]
struct Foo {
    x: u32
}

impl PartialEq for Foo {
    fn eq(&self, _: &Foo) -> bool {
        false // ha ha!
    }
}

const FOO: Foo = Foo { x: 0 };

fn main() {
    let y = Foo { x: 1 };
    match y {
        FOO => { }
        //~^ ERROR must be annotated with `#[derive(PartialEq)]`
        _ => { }
    }
}
