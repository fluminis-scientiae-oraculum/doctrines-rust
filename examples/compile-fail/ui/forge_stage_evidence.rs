use staged_protocol::{ConsentProof, PolicyVersion};

fn main() {
    let version = PolicyVersion(3);
    ConsentProof { version };
}
