/// A macro that quickly generates wrapper functions over a "metadata" field.
///
/// # Example
///
/// ```rust
/// use froglight_registry_template::implement_wrapper;
///
/// pub struct Inner;
///
/// impl Inner {
///     pub fn do_something(&self, arg: u32) -> u64 { todo!() }
/// }
///
/// pub struct Wrapper {
///     value: u32,
///     meta: &'static Inner,
/// }
///
/// implement_wrapper! {
///    impl Wrapper {
///        [ value => meta ]
///
///        #[must_use]
///        pub fn do_something(&self) -> u64;
///    }
/// }
///
/// // Becomes (renamed due to name collision):
///
/// impl Wrapper {
///     #[must_use]
///     pub fn _do_something(&self) -> u64 { self.meta.do_something(self.value) }
/// }
/// ```
#[macro_export]
macro_rules! implement_wrapper {
    (
        impl $outer:ident {
            [ () => $meta:tt ]

            $(
                $( #[$fn_attr:meta] )*
                $vis:vis fn $fn_name:ident

                $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)?

                (&self $(, $fn_arg:ident : $fn_arg_ty:ty )* )

                -> $fn_return:ty;
            )*
        }
    ) => {
        impl $outer {
            $(
                $( #[$fn_attr] )*
                $vis fn $fn_name

                $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?

                (&self $(, $fn_arg : $fn_arg_ty )* )

                -> $fn_return {
                    self.$meta.$fn_name

                    $( $(::< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? )?

                    ( $( $fn_arg ),* )
                }
            )*
        }
    };

    (
        impl $outer:ident {
            [ $arg:ident => $meta:tt ]

            $(
                $( #[$fn_attr:meta] )*
                $vis:vis fn $fn_name:ident

                $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)?

                (&self $(, $fn_arg:ident : $fn_arg_ty:ty )* )

                -> $fn_return:ty;
            )*
        }
    ) => {
        impl $outer {
            $(
                $( #[$fn_attr] )*
                $vis fn $fn_name

                $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?

                (&self $(, $fn_arg : $fn_arg_ty )* )

                -> $fn_return {
                    self.$meta.$fn_name

                    $( $(::< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? )?

                    (self.$arg $(, $fn_arg ),* )
                }
            )*
        }
    };
    (
        impl $outer:ident {
            [ $arg:ident => $meta:tt() ]

            $(
                $( #[$fn_attr:meta] )*
                $vis:vis fn $fn_name:ident

                $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)?

                (&self $(, $fn_arg:ident : $fn_arg_ty:ty )* )

                -> $fn_return:ty;
            )*
        }
    ) => {
        impl $outer {
            $(
                $( #[$fn_attr] )*
                $vis fn $fn_name $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? (&self) -> $fn_return {
                    self.$meta().$fn_name

                    $( $(::< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? )?

                    (self.$arg $(, $fn_arg ),* )
                }
            )*
        }
    };

    (
        impl $outer:ident {
            [ $arg:ident => $meta:tt.$meta2:tt ]

            $(
                $( #[$fn_attr:meta] )*
                $vis:vis fn $fn_name:ident

                $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)?

                (&self $(, $fn_arg:ident : $fn_arg_ty:ty )* )

                -> $fn_return:ty;
            )*
        }
    ) => {
        impl $outer {
            $(
                $( #[$fn_attr] )*
                $vis fn $fn_name $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? (&self) -> $fn_return {
                    self.$meta.$meta2.$fn_name

                    $( $(::< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? )?

                    (self.$arg $(, $fn_arg ),* )
                }
            )*
        }
    };
    (
        impl $outer:ident {
            [ $arg:ident => $meta:tt.$meta2:tt() ]

            $(
                $( #[$fn_attr:meta] )*
                $vis:vis fn $fn_name:ident

                $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)?

                (&self $(, $fn_arg:ident : $fn_arg_ty:ty )* )

                -> $fn_return:ty;
            )*
        }
    ) => {
        impl $outer {
            $(
                $( #[$fn_attr] )*
                $vis fn $fn_name $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? (&self) -> $fn_return {
                    self.$meta.$meta2().$fn_name

                    $( $(::< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? )?

                    (self.$arg $(, $fn_arg ),* )
                }
            )*
        }
    };
}
