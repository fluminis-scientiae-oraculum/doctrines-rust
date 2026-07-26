use doctrine_compile_fail::{Closed, Connection, Open};

fn main() {
    let mut connection = Connection::<Closed>::new();
    let _receipt = Connection::<Open>::send(&mut connection, b"must not compile");
}
