use doctrine_compile_fail::Transaction;

fn main() {
    let mut transaction = Transaction::begin();
    transaction.stage("first mutation");
    let _receipt = transaction.commit();
    transaction.stage("reuse after commit must not compile");
}
