use staged_protocol::{Canonicalize, CheckIdentity, IdentityDirectory, RawSubmission, SelfServiceSubmission};

fn main() {
    let directory = IdentityDirectory::new();
    let submission = RawSubmission { address: "applicant@example.com".to_owned(), display_name: "Example Applicant".to_owned() };
    let canonical = SelfServiceSubmission { submission, challenge_id: "challenge-1".to_owned() }.canonicalize().expect("canonical values");
    let _first = canonical.check_identity(&directory);
    let _second = canonical.check_identity(&directory);
}
