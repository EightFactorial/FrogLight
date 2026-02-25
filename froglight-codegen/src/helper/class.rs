use cafebabe::{
    ClassFile, MethodInfo,
    attributes::{AttributeData, BootstrapMethodEntry, CodeData},
    bytecode::{ByteCode, Opcode},
    constant_pool::{BootstrapArgument, MemberRef, MethodHandle},
};
use miette::Result;

use crate::source::{JarData, JarFile};

pub trait ClassFileExt {
    fn get_method(&self, name: &str) -> Option<&MethodInfo<'static>>;

    fn get_bootstrap(&self) -> Option<&[BootstrapMethodEntry<'static>]>;

    fn get_method_code(&self, name: &str) -> Option<&CodeData<'static>> {
        let method = self.get_method(name)?;
        let code_attr = method.attributes.iter().find(|a| a.name == "Code")?;
        if let AttributeData::Code(code) = &code_attr.data { Some(code) } else { None }
    }

    fn get_static_code(&self) -> Option<&CodeData<'static>> { self.get_method_code("<clinit>") }

    fn get_static_field_init(&self, field: &str) -> Option<Vec<Opcode<'static>>>;

    fn iterate_code<F: FnMut(usize, &Opcode<'static>) -> Result<()>>(
        &self,
        code: &ByteCode<'static>,
        jar: &JarData,
        depth: usize,
        f: &mut F,
    ) -> Result<()>;
}

impl ClassFileExt for ClassFile<'static> {
    fn get_method(&self, name: &str) -> Option<&MethodInfo<'static>> {
        self.methods.iter().find(|m| m.name == name)
    }

    fn get_bootstrap(&self) -> Option<&[BootstrapMethodEntry<'static>]> {
        self.attributes.iter().find(|attr| attr.name == "BootstrapMethods").and_then(|attr| {
            if let AttributeData::BootstrapMethods(methods) = &attr.data {
                Some(methods.as_slice())
            } else {
                None
            }
        })
    }

    fn get_static_field_init(&self, field: &str) -> Option<Vec<Opcode<'static>>> {
        let code = self.get_static_code()?;

        let mut init_ops = Vec::new();
        for (_, op) in &code.bytecode.as_ref().unwrap().opcodes {
            if let Opcode::Putstatic(MemberRef { class_name, name_and_type }) = op {
                // Ignore fields of other classes
                if class_name != &*self.this_class {
                    continue;
                }

                if name_and_type.name == field {
                    // Stop collecting once we initialize the target field
                    break;
                }

                // Clear any operations used for other fields
                init_ops.clear();
            }
            init_ops.push(op.clone());
        }

        Some(init_ops)
    }

    fn iterate_code<F: FnMut(usize, &Opcode<'static>) -> Result<()>>(
        &self,
        code: &ByteCode<'static>,
        jar: &JarData,
        depth: usize,
        f: &mut F,
    ) -> Result<()> {
        if depth > 16 {
            miette::bail!("Maximum bootstrap method depth exceeded");
        }

        let process = |method: &MethodHandle<'static>, f: &mut F| -> Result<()> {
            if !method.class_name.starts_with("net/minecraft") {
                return Ok(()); // Skip outside methods
            }

            if let Some(method_code) = jar.get_class_method_code(
                &method.class_name,
                &method.member_ref.name,
                Some(&method.member_ref.descriptor),
            ) {
                // Continue iterating through the dynamic method's code
                let bytecode = method_code.bytecode.as_ref().unwrap();
                self.iterate_code(bytecode, jar, depth + 1, f)
            } else {
                miette::bail!(
                    "Bootstrap method not found: {}.{}",
                    method.class_name,
                    method.member_ref.name
                );
            }
        };

        for (index, opcode) in &code.opcodes {
            f(*index, opcode)?;

            if let Opcode::Invokedynamic(invoke) = opcode {
                let dynamic = self.get_bootstrap().unwrap();
                let entry = &dynamic[invoke.attr_index as usize];

                // Process the invokedynamic bootstrap method
                (process)(&entry.method, f)?;

                // Process any additional method arguments
                for arg in &entry.arguments {
                    if let BootstrapArgument::MethodHandle(method) = arg {
                        (process)(method, f)?;
                    }
                }
            }
        }

        Ok(())
    }
}
