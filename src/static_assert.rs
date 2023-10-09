//! Compile-time assertions.

// Taken from https://rust-for-linux.github.io/docs/src/kernel/static_assert.rs.html.

macro_rules! static_assert {
    ($condition:expr) => {
        const _: () = core::assert!($condition);
    };
}

pub(crate) use static_assert;
