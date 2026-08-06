#![deny(unsafe_op_in_unsafe_fn)]
#![doc = include_str!("../README.md")]

use core::mem::MaybeUninit;
use core::ptr;

struct InitializedPrefix<T> {
    first: *mut T,
    initialized: usize,
}

impl<T> Drop for InitializedPrefix<T> {
    fn drop(&mut self) {
        let initialized = ptr::slice_from_raw_parts_mut(self.first, self.initialized);

        // SAFETY: `first` retains provenance from live, aligned
        // `MaybeUninit<[T; N]>` storage owned by the calling function. Exactly
        // `initialized` logical prefix slots contain valid `T`, including for
        // zero-sized `T`. No reference or competing drop owner aliases that
        // prefix, and the guard cannot outlive the storage.
        unsafe {
            ptr::drop_in_place(initialized);
        }
    }
}

/// Fallibly initializes an array without allocating.
///
/// On success, the returned array owns all `N` values. On error, every value
/// produced before the error is dropped exactly once.
///
/// # Errors
///
/// Returns the first error produced by `build`.
///
/// # Panics
///
/// Propagates a panic from `build` after dropping the initialized prefix. A
/// panic from `T::drop` follows Rust's ordinary unwinding behavior.
///
/// # Safety argument
///
/// The implementation writes each in-bounds slot once, tracks the initialized
/// prefix in a private drop guard, and calls `assume_init` only after all `N`
/// slots have been written. Safe callers cannot alter that bookkeeping.
pub fn try_init_array<T, E, const N: usize>(
    mut build: impl FnMut(usize) -> Result<T, E>,
) -> Result<[T; N], E> {
    let mut storage = MaybeUninit::<[T; N]>::uninit();
    let first = storage.as_mut_ptr().cast::<T>();
    let mut guard = InitializedPrefix {
        first,
        initialized: 0,
    };

    for index in 0..N {
        let value = build(index)?;

        // SAFETY: `index < N`; `first` retains provenance from aligned, live
        // storage for `[T; N]`. `add(index)` stays within that array allocation,
        // including the zero-sized case.
        let slot = unsafe { first.add(index) };

        // SAFETY: the logical slot is exactly `guard.initialized`, so it is
        // uninitialized and has not been written, referenced, or assigned
        // another drop owner. `value` is a valid owned `T`.
        unsafe {
            slot.write(value);
        }
        guard.initialized += 1;
    }

    debug_assert_eq!(guard.initialized, N);
    guard.initialized = 0;

    // SAFETY: the loop completed, so all `N` logical slots contain valid `T`
    // values and `[T; N]` is fully initialized. The guard has been disarmed;
    // ownership transfers to the returned array without a live alias.
    Ok(unsafe { storage.assume_init() })
}

#[cfg(test)]
mod tests {
    use super::try_init_array;
    use std::cell::{Cell, RefCell};
    use std::convert::Infallible;
    use std::mem;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::rc::Rc;

    struct DropProbe {
        index: usize,
        dropped: Rc<RefCell<Vec<usize>>>,
    }

    impl Drop for DropProbe {
        fn drop(&mut self) {
            self.dropped.borrow_mut().push(self.index);
        }
    }

    fn probe(index: usize, dropped: &Rc<RefCell<Vec<usize>>>) -> DropProbe {
        DropProbe {
            index,
            dropped: Rc::clone(dropped),
        }
    }

    #[test]
    fn success_transfers_every_element_to_the_returned_array() {
        let dropped = Rc::new(RefCell::new(Vec::new()));
        let array = try_init_array::<_, Infallible, 3>(|index| Ok(probe(index, &dropped)))
            .expect("infallible builder");

        assert_eq!(array[0].index, 0);
        assert_eq!(array[1].index, 1);
        assert_eq!(array[2].index, 2);
        assert!(dropped.borrow().is_empty());

        drop(array);
        assert_eq!(*dropped.borrow(), [0, 1, 2]);
    }

    #[derive(Debug, Eq, PartialEq)]
    struct BuildError;

    #[test]
    fn error_drops_only_the_initialized_prefix() {
        let dropped = Rc::new(RefCell::new(Vec::new()));
        let result = try_init_array::<_, BuildError, 4>(|index| {
            if index == 2 {
                Err(BuildError)
            } else {
                Ok(probe(index, &dropped))
            }
        });

        assert!(matches!(result, Err(BuildError)));
        assert_eq!(*dropped.borrow(), [0, 1]);
    }

    #[test]
    fn panic_drops_only_the_initialized_prefix() {
        let dropped = Rc::new(RefCell::new(Vec::new()));
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            let _array = try_init_array::<_, Infallible, 4>(|index| {
                assert_ne!(index, 2, "builder panic");
                Ok(probe(index, &dropped))
            });
        }));

        assert!(outcome.is_err());
        assert_eq!(*dropped.borrow(), [0, 1]);
    }

    #[test]
    fn zero_length_never_calls_the_builder() {
        let mut calls = 0;
        let array = try_init_array::<u8, Infallible, 0>(|_| {
            calls += 1;
            Ok(7)
        })
        .expect("infallible builder");

        assert_eq!(calls, 0);
        assert_eq!(array, []);
    }

    std::thread_local! {
        static ZERO_SIZED_DROPS: Cell<usize> = const { Cell::new(0) };
    }

    struct ZeroSizedDrop;

    impl Drop for ZeroSizedDrop {
        fn drop(&mut self) {
            ZERO_SIZED_DROPS.set(ZERO_SIZED_DROPS.get() + 1);
        }
    }

    #[test]
    fn zero_sized_values_drop_by_logical_array_length() {
        assert_eq!(mem::size_of::<ZeroSizedDrop>(), 0);
        ZERO_SIZED_DROPS.set(0);

        let array =
            try_init_array::<_, Infallible, 4>(|_| Ok(ZeroSizedDrop)).expect("infallible builder");
        assert_eq!(ZERO_SIZED_DROPS.get(), 0);

        drop(array);
        assert_eq!(ZERO_SIZED_DROPS.get(), 4);
    }
}
