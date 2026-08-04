use staged_protocol::{AvailableRegistration, Canonicalize, RawSubmission, SelfServiceOrigin, SelfServiceSubmission};

fn policy_stage(_stage: AvailableRegistration<SelfServiceOrigin>) {}

fn main() {
    let submission = RawSubmission { address: "applicant@example.com".to_owned(), display_name: "Example Applicant".to_owned() };
    let canonical = SelfServiceSubmission { submission, challenge_id: "challenge-1".to_owned() }.canonicalize().expect("canonical values");
    policy_stage(canonical);
}
