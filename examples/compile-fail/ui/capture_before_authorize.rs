use doctrine_compile_fail::{Authorized, Draft, Payment};
use std::num::NonZeroU64;

fn main() {
    let amount = NonZeroU64::new(10).expect("ten is nonzero");
    let payment = Payment::<Draft>::new(amount);
    let _receipt = Payment::<Authorized>::capture(payment);
}
