use std::marker::PhantomData;

use cafebabe::{
    attributes::{AttributeData, AttributeInfo, CodeData},
    bytecode::Opcode,
};
use miette::Result;

use crate::source::JarData;

pub struct BytecodeEmulator<'a> {
    _phantom: PhantomData<&'a ()>,
}

impl<'a> BytecodeEmulator<'a> {
    /// Run the bytecode emulator starting from the specified class and method.
    pub fn run(jar: &'a JarData, class: &str, method: &str) -> Result<Self> {
        let mut emulator = BytecodeEmulator { _phantom: PhantomData };

        // Find the method info and ensure it takes no parameters
        let Some(method_info) = jar.get_class_method(class, method) else {
            miette::bail!("Could not find entrypoint `{class}.{method}`");
        };
        if !method_info.descriptor.parameters.is_empty() {
            miette::bail!("Entrypoint `{class}.{method}` cannot have parameters!");
        }

        // Find the method's code
        let Some(AttributeInfo { data: AttributeData::Code(code), .. }) =
            method_info.attributes.iter().find(|a| a.name == "Code")
        else {
            miette::bail!("Could not find code for entrypoint `{class}.{method}`");
        };
        tracing::debug!("Emulating `{class}.{method}`...");

        // Run the emulator
        emulator.emulate(jar, iter_bytecode(code))?;

        Ok(emulator)
    }

    #[expect(clippy::unused_self, reason = "WIP")]
    #[expect(clippy::match_same_arms, clippy::too_many_lines, reason = "Readability")]
    fn emulate(&mut self, _jar: &JarData, ops: impl Iterator<Item = &'a Opcode<'a>>) -> Result<()> {
        for op in ops {
            tracing::trace!("Opcode: {op:?}");

            match op {
                Opcode::Aaload => {}
                Opcode::Aastore => {}
                Opcode::AconstNull => {}
                Opcode::Aload(_) => {}
                Opcode::Anewarray(_) => {}
                Opcode::Areturn => {}
                Opcode::Arraylength => {}
                Opcode::Astore(_) => {}
                Opcode::Athrow => {}
                Opcode::Baload => {}
                Opcode::Bastore => {}
                Opcode::Bipush(_) => {}
                Opcode::Breakpoint => {}
                Opcode::Caload => {}
                Opcode::Castore => {}
                Opcode::Checkcast(_) => {}
                Opcode::D2f => {}
                Opcode::D2i => {}
                Opcode::D2l => {}
                Opcode::Dadd => {}
                Opcode::Daload => {}
                Opcode::Dastore => {}
                Opcode::Dcmpg => {}
                Opcode::Dcmpl => {}
                Opcode::Dconst0 => {}
                Opcode::Dconst1 => {}
                Opcode::Ddiv => {}
                Opcode::Dload(_) => {}
                Opcode::Dmul => {}
                Opcode::Dneg => {}
                Opcode::Drem => {}
                Opcode::Dreturn => {}
                Opcode::Dstore(_) => {}
                Opcode::Dsub => {}
                Opcode::Dup => {}
                Opcode::DupX1 => {}
                Opcode::DupX2 => {}
                Opcode::Dup2 => {}
                Opcode::Dup2X1 => {}
                Opcode::Dup2X2 => {}
                Opcode::F2d => {}
                Opcode::F2i => {}
                Opcode::F2l => {}
                Opcode::Fadd => {}
                Opcode::Faload => {}
                Opcode::Fastore => {}
                Opcode::Fcmpg => {}
                Opcode::Fcmpl => {}
                Opcode::Fconst0 => {}
                Opcode::Fconst1 => {}
                Opcode::Fconst2 => {}
                Opcode::Fdiv => {}
                Opcode::Fload(_) => {}
                Opcode::Fmul => {}
                Opcode::Fneg => {}
                Opcode::Frem => {}
                Opcode::Freturn => {}
                Opcode::Fstore(_) => {}
                Opcode::Fsub => {}
                Opcode::Getfield(_) => {}
                Opcode::Getstatic(_) => {}
                Opcode::Goto(_) => {}
                Opcode::I2b => {}
                Opcode::I2c => {}
                Opcode::I2d => {}
                Opcode::I2f => {}
                Opcode::I2l => {}
                Opcode::I2s => {}
                Opcode::Iadd => {}
                Opcode::Iaload => {}
                Opcode::Iand => {}
                Opcode::Iastore => {}
                Opcode::IconstM1 => {}
                Opcode::Iconst0 => {}
                Opcode::Iconst1 => {}
                Opcode::Iconst2 => {}
                Opcode::Iconst3 => {}
                Opcode::Iconst4 => {}
                Opcode::Iconst5 => {}
                Opcode::Idiv => {}
                Opcode::IfAcmpeq(_) => {}
                Opcode::IfAcmpne(_) => {}
                Opcode::IfIcmpeq(_) => {}
                Opcode::IfIcmpge(_) => {}
                Opcode::IfIcmpgt(_) => {}
                Opcode::IfIcmple(_) => {}
                Opcode::IfIcmplt(_) => {}
                Opcode::IfIcmpne(_) => {}
                Opcode::Ifeq(_) => {}
                Opcode::Ifge(_) => {}
                Opcode::Ifgt(_) => {}
                Opcode::Ifle(_) => {}
                Opcode::Iflt(_) => {}
                Opcode::Ifne(_) => {}
                Opcode::Ifnonnull(_) => {}
                Opcode::Ifnull(_) => {}
                Opcode::Iinc(..) => {}
                Opcode::Iload(_) => {}
                Opcode::Impdep1 => {}
                Opcode::Impdep2 => {}
                Opcode::Imul => {}
                Opcode::Ineg => {}
                Opcode::Instanceof(_) => {}
                Opcode::Invokedynamic(_) => {}
                Opcode::Invokeinterface(..) => {}
                Opcode::Invokespecial(_) => {}
                Opcode::Invokestatic(_) => {}
                Opcode::Invokevirtual(_) => {}
                Opcode::Ior => {}
                Opcode::Irem => {}
                Opcode::Ireturn => {}
                Opcode::Ishl => {}
                Opcode::Ishr => {}
                Opcode::Istore(_) => {}
                Opcode::Isub => {}
                Opcode::Iushr => {}
                Opcode::Ixor => {}
                Opcode::Jsr(_) => {}
                Opcode::L2d => {}
                Opcode::L2f => {}
                Opcode::L2i => {}
                Opcode::Ladd => {}
                Opcode::Laload => {}
                Opcode::Land => {}
                Opcode::Lastore => {}
                Opcode::Lcmp => {}
                Opcode::Lconst0 => {}
                Opcode::Lconst1 => {}
                Opcode::Ldc(_) => {}
                Opcode::LdcW(_) => {}
                Opcode::Ldc2W(_) => {}
                Opcode::Ldiv => {}
                Opcode::Lload(_) => {}
                Opcode::Lmul => {}
                Opcode::Lneg => {}
                Opcode::Lookupswitch(_) => {}
                Opcode::Lor => {}
                Opcode::Lrem => {}
                Opcode::Lreturn => {}
                Opcode::Lshl => {}
                Opcode::Lshr => {}
                Opcode::Lstore(_) => {}
                Opcode::Lsub => {}
                Opcode::Lushr => {}
                Opcode::Lxor => {}
                Opcode::Monitorenter => {}
                Opcode::Monitorexit => {}
                Opcode::Multianewarray(..) => {}
                Opcode::New(_) => {}
                Opcode::Newarray(_) => {}
                Opcode::Nop => {}
                Opcode::Pop => {}
                Opcode::Pop2 => {}
                Opcode::Putfield(_) => {}
                Opcode::Putstatic(_) => {}
                Opcode::Ret(_) => {}
                Opcode::Return => {}
                Opcode::Saload => {}
                Opcode::Sastore => {}
                Opcode::Sipush(_) => {}
                Opcode::Swap => {}
                Opcode::Tableswitch(_) => {}
            }
        }

        Ok(())
    }
}

/// Iterate over the opcodes in the given [`CodeData`].
fn iter_bytecode<'a>(code: &'a CodeData<'a>) -> impl Iterator<Item = &'a Opcode<'a>> {
    code.bytecode.as_ref().unwrap().opcodes.iter().map(|(_, op)| op)
}
