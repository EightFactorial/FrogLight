macro_rules! impl_tuple_var_read {
    ($($name:ident),*) => {
        impl<$($name: crate::protocol::FrogVarRead),*> crate::protocol::FrogVarRead for ($($name,)*) {
            fn fg_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::protocol::ReadError>
            where
                Self: Sized,
            {
                Ok(($($name::fg_var_read(buf)?,)*))
            }
        }
    };
}

impl_tuple_var_read!(A, B);
impl_tuple_var_read!(A, B, C);
impl_tuple_var_read!(A, B, C, D);
impl_tuple_var_read!(A, B, C, D, E);
impl_tuple_var_read!(A, B, C, D, E, F);
impl_tuple_var_read!(A, B, C, D, E, F, G);
impl_tuple_var_read!(A, B, C, D, E, F, G, H);
impl_tuple_var_read!(A, B, C, D, E, F, G, H, I);
impl_tuple_var_read!(A, B, C, D, E, F, G, H, I, J);
impl_tuple_var_read!(A, B, C, D, E, F, G, H, I, J, K);
