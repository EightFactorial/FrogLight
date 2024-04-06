#![allow(non_snake_case)]

macro_rules! impl_tuple_write {
    ($($name:ident),*) => {
        impl<$($name: crate::protocol::FrogWrite),*> crate::protocol::FrogWrite for ($($name,)*) {
            fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), crate::protocol::WriteError> {
                let ($($name,)*) = self;
                $($name.fg_write(buf)?;)*
                Ok(())
            }
        }
    };
}

impl_tuple_write!(A, B);
impl_tuple_write!(A, B, C);
impl_tuple_write!(A, B, C, D);
impl_tuple_write!(A, B, C, D, E);
impl_tuple_write!(A, B, C, D, E, F);
impl_tuple_write!(A, B, C, D, E, F, G);
impl_tuple_write!(A, B, C, D, E, F, G, H);
impl_tuple_write!(A, B, C, D, E, F, G, H, I);
impl_tuple_write!(A, B, C, D, E, F, G, H, I, J);
impl_tuple_write!(A, B, C, D, E, F, G, H, I, J, K);
