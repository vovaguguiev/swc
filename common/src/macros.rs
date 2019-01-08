/// Create a impl block with some bitflags.
#[macro_export]
macro_rules! make_impl_block {
    (
        @NAME,
        Done {
            $($done:tt)*
        },

        $(#[$attr:meta])*
        $name:ident:

        $($rest:tt)+
    ) => {
        make_impl_block!(
            @VALUE,
            Cur {},
            Done {
                $($done)*

                $(#[$attr])*
                $name :
            },

            $($rest)*
        );
    };

    // Handle comma.
    (
        @VALUE,
        Cur {
            $($cur:tt)+
        },
        Done {
            $($done:tt)*
        },

        ,

        $($rest:tt)*
    ) => {
        make_impl_block!(
            @NAME,
            Done {
                $($done)*

                ( $($cur)* ),
            },

            $($rest)*
        );
    };

    // Handle value.
    (
        @VALUE,
        Cur {
            $($cur:tt)*
        },
        Done {
            $($done:tt)*
        },

        $tok:tt

        $($rest:tt)+
    ) => {
        make_impl_block!(
            @VALUE,
            Cur {
                $($cur)*
                $tok
            },
            Done {
                $($done)*
            },

            $($rest)*
        );
    };

    // Handle value without trailing comma.
    (
        @VALUE,
        Cur {
            $($cur:tt)*
        },
        Done {
            $($done:tt)*
        },

        $tok:tt
    ) => {
        make_impl_block!(
            @VALUE,
            Cur {
                $($cur)*
                $tok
            },
            Done {
                $($done)*
            },

            ,
        );
    };




    // Done.
    (
        @NAME,
        Done {
            $STRUCT:ident;
            $(
                $(#[$attr:meta])*
                $name:ident: ( $($val:tt)* ),
            )*
        },
    ) => {
        impl $STRUCT {
            $(
                $(#[$attr])*
                pub const $name: Self = $STRUCT { bits: value_of_bitflag!($($val)*) };
            )*
        }
    };
}

#[macro_export]
macro_rules! value_of_bitflag {
    (
        $i:ident |
        $($rest:tt)*
    ) => {
        Self::$i.bits | value_of_bitflag!($($rest)*)
    };

    (
        $i:ident
    ) => {
        Self::$i.bits
    };

    ($e:expr) => { $e };
}

/// rustfmt-friendly version of `bitblags!`.
#[macro_export]
macro_rules! add_bitflags {
    // Done
    ($STRUCT:ident,) => {};

    (
        $STRUCT:ident,
        $(#[$attr:meta])*
        Values {
            $($tt:tt)*
        },

        $($rest:tt)*
    ) => {
        $(#[$attr])*
        make_impl_block!(@NAME, Done {
            $STRUCT;
        }, $($tt)*);

        add_bitflags!(
            $STRUCT,
            $($rest)*
        );
    };
}
