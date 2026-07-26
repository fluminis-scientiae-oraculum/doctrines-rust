use doctrine_compile_fail::VerifiedEmailAddress;

fn main() {
    let _forged = VerifiedEmailAddress::from_accepted_evidence(
        "owner@example.com".to_owned(),
        "forged".to_owned(),
    );
}
