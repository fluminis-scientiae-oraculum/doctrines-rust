use staged_protocol::{RawSubmission, SelfServiceSubmission};

fn main() {
    let submission = RawSubmission { address: "applicant@example.com".to_owned(), display_name: "Example Applicant".to_owned() };
    let entry = SelfServiceSubmission { submission, challenge_id: "challenge-1".to_owned() };
    let _twin = entry.clone();
}
