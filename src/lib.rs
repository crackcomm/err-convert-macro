//! Errors converting macros.

#[macro_export]
macro_rules! err_converter {
    ( $a:ident, $b:ty ) => {
        impl From<$b> for Error {
            #[inline(always)]
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
            #[inline(always)]
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
            #[inline(always)]
            fn from(e: $b) -> Self {
                Error::$a(e.into())
            }
        }
    };
}

#[macro_export]
macro_rules! err_convert_into_box {
    () => {
        impl From<Error> for Box<dyn std::error::Error + Send + Sync + 'static> {
            #[inline(always)]
            fn from(error: Error) -> Self {
                use failure::Fail;
                error.compat().into()
            }
        }
    };
}
