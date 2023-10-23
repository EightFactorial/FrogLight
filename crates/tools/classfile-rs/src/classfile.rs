use crate::{
    access::ClassAccessFlags,
    attributes::{Attribute, AttributeSource, Attributes},
    constantpool::{ConstantPool, ConstantPoolWriter},
    error::{ParserError, Result},
    field::{Field, Fields},
    method::{Method, Methods},
    version::ClassVersion,
    Serializable,
};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read, Write};

#[derive(Clone, Debug, PartialEq)]
pub struct ClassFile {
    /// 0xCAFEBABE
    pub magic: u32,
    pub version: ClassVersion,
    pub access_flags: ClassAccessFlags,
    pub this_class: String,
    /// Can be None for example for java/lang/Object
    pub super_class: Option<String>,
    pub interfaces: Vec<String>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}

impl ClassFile {
    pub fn parse<R: Read>(rdr: &mut R) -> Result<Self> {
        let magic = rdr.read_u32::<BigEndian>()?;
        if magic != 0xCAFEBABE {
            return Err(ParserError::unrecognised("header", magic.to_string()));
        }
        let version = ClassVersion::parse(rdr)?;
        let constant_pool = ConstantPool::parse(rdr)?;
        let access_flags = ClassAccessFlags::parse(rdr)?;
        let this_class = constant_pool
            .utf8(
                constant_pool
                    .class(rdr.read_u16::<BigEndian>()?)?
                    .name_index,
            )?
            .str
            .clone();
        let super_class = match rdr.read_u16::<BigEndian>()? {
            0 => None,
            i => Some(
                constant_pool
                    .utf8(constant_pool.class(i)?.name_index)?
                    .str
                    .clone(),
            ),
        };

        let num_interfaces = rdr.read_u16::<BigEndian>()? as usize;
        let mut interfaces: Vec<String> = Vec::with_capacity(num_interfaces);
        for _ in 0..num_interfaces {
            interfaces.push(
                constant_pool
                    .utf8(
                        constant_pool
                            .class(rdr.read_u16::<BigEndian>()?)?
                            .name_index,
                    )?
                    .str
                    .clone(),
            );
        }

        let fields = Fields::parse(rdr, &version, &constant_pool)?;
        let methods = Methods::parse(rdr, &version, &constant_pool)?;
        let attributes = Attributes::parse(
            rdr,
            AttributeSource::Class,
            &version,
            &constant_pool,
            &mut None,
        )?;

        Ok(ClassFile {
            magic,
            version,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }

    pub fn write<W: Write>(&self, wtr: &mut W) -> Result<()> {
        wtr.write_u32::<BigEndian>(self.magic)?;
        self.version.write(wtr)?;

        let mut constant_pool = ConstantPoolWriter::new();

        // we need to write fields/methods etc after the constant pool, however they rely upon
        // mutable access to the constant pool. therefore we will write them to memory and then to
        // the wtr parameter
        let buf: Vec<u8> =
            Vec::with_capacity(2 + (self.fields.len() * 8) + (self.methods.len() * 8));
        let mut cursor = Cursor::new(buf);
        self.access_flags.write(&mut cursor)?;

        // this class
        let utf = constant_pool.utf8(self.this_class.clone());
        cursor.write_u16::<BigEndian>(constant_pool.class(utf))?;
        // super class
        if let Some(x) = &self.super_class {
            let utf = constant_pool.utf8(x.clone());
            cursor.write_u16::<BigEndian>(constant_pool.class(utf))?;
        } else {
            cursor.write_u16::<BigEndian>(0)?;
        }
        // interfaces
        cursor.write_u16::<BigEndian>(self.interfaces.len() as u16)?;
        for interface in self.interfaces.iter() {
            let utf = constant_pool.utf8(interface.clone());
            cursor.write_u16::<BigEndian>(constant_pool.class(utf))?;
        }

        Fields::write(&mut cursor, &self.fields, &mut constant_pool)?;
        Methods::write(&mut cursor, &self.methods, &mut constant_pool)?;
        Attributes::write(&mut cursor, &self.attributes, &mut constant_pool, None)?;

        constant_pool.write(wtr)?;
        wtr.write_all(cursor.get_ref().as_slice())?;

        Ok(())
    }
}
