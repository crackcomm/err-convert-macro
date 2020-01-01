//! Errors converting macros.

#[macro_export]
macro_rules! err_converter {
    ( $a:ident, $b:ty ) => {
        impl From<$b> for Error {
            fn from(e: $b) -> Self {
                Error::$a(e)
            }
        }
    };
}

#[macro_export]
macro_rules! err_converter_no_args {
    ( $a:ident, $b:ty ) => {
        impl From<$b> for Error {
            fn from(_: $b) -> Self {
                Error::$a
            }
        }
    };
}

#[macro_export]
macro_rules! err_converter_with_into {
    ( $a:ident, $b:ty ) => {
        impl From<$b> for Error {
            fn from(e: $b) -> Self {
                Error::$a(e.into())
            }
        }
    };
}
