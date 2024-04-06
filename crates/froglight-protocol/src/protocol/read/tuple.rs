macro_rules! impl_tuple_read {
    ($($name:ident),*) => {
        impl<$($name: crate::protocol::FrogRead),*> crate::protocol::FrogRead for ($($name,)*) {
            fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::protocol::ReadError>
            where
                Self: Sized,
            {
                Ok(($($name::fg_read(buf)?,)*))
            }
        }
    };
}

impl_tuple_read!(A, B);
impl_tuple_read!(A, B, C);
impl_tuple_read!(A, B, C, D);
impl_tuple_read!(A, B, C, D, E);
impl_tuple_read!(A, B, C, D, E, F);
impl_tuple_read!(A, B, C, D, E, F, G);
impl_tuple_read!(A, B, C, D, E, F, G, H);
impl_tuple_read!(A, B, C, D, E, F, G, H, I);
impl_tuple_read!(A, B, C, D, E, F, G, H, I, J);
impl_tuple_read!(A, B, C, D, E, F, G, H, I, J, K);
