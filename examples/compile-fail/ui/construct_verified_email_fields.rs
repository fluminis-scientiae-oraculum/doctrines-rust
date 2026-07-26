use doctrine_compile_fail::VerifiedEmailAddress;

fn main() {
    let evidence = (
        "owner@example.com".to_owned(),
        "forged-verification".to_owned(),
    );
    VerifiedEmailAddress { evidence };
}
