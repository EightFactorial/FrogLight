#![allow(non_snake_case)]

macro_rules! impl_tuple_var_write {
    ($($name:ident),*) => {
        impl<$($name: crate::io::FrogVarWrite),*> crate::io::FrogVarWrite for ($($name,)*) {
            fn fg_var_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), crate::io::WriteError> {
                let ($($name,)*) = self;
                $($name.fg_var_write(buf)?;)*
                Ok(())
            }
        }
    };
}

impl_tuple_var_write!(A, B);
impl_tuple_var_write!(A, B, C);
impl_tuple_var_write!(A, B, C, D);
impl_tuple_var_write!(A, B, C, D, E);
impl_tuple_var_write!(A, B, C, D, E, F);
impl_tuple_var_write!(A, B, C, D, E, F, G);
impl_tuple_var_write!(A, B, C, D, E, F, G, H);
impl_tuple_var_write!(A, B, C, D, E, F, G, H, I);
impl_tuple_var_write!(A, B, C, D, E, F, G, H, I, J);
impl_tuple_var_write!(A, B, C, D, E, F, G, H, I, J, K);
