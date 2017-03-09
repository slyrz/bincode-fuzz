use std::io::{Write, Result};
use std::path::Path;
use std::fs::File;

use random::{Rng, Rand};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Type {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Char,
    Bool,
    String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CompoundType {
    Vec(Type),
    Option(Type),
    Tuple(Type, Type),
    Array(Type, usize),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StructField {
    Type(Type),
    CompoundType(CompoundType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct(Vec<StructField>);


impl Struct {
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut file = File::create(path)?;

        let mut fields = String::new();
        for (i, field) in self.0.iter().enumerate() {
            let line = format!("field_{:02x}: {},\n", i, field.to_type());
            fields.push_str(&line);
        }

        let mut generators = String::new();
        for (i, field) in self.0.iter().enumerate() {
            let line = format!("field_{:02x}: {},\n", i, field.to_generator());
            generators.push_str(&line);
        }

        write!(file,
               r#"
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {{
    {}
}}

impl Rand for Test {{
    fn rand<R: Rng>(rng: &mut R) -> Test {{
        Test {{
            {}
        }}
    }}
}}"#,
               fields,
               generators)?;
        Ok(())
    }
}

trait ToType {
    fn to_type(&self) -> String;
}

impl ToType for Type {
    fn to_type(&self) -> String {
        let name = match *self {
            Type::U8 => "u8",
            Type::U16 => "u16",
            Type::U32 => "u32",
            Type::U64 => "u64",
            Type::I8 => "i8",
            Type::I16 => "i16",
            Type::I32 => "i32",
            Type::I64 => "i64",
            Type::F32 => "f32",
            Type::F64 => "f64",
            Type::Char => "char",
            Type::Bool => "bool",
            Type::String => "String",
        };
        name.to_string()
    }
}

impl ToType for CompoundType {
    fn to_type(&self) -> String {
        match *self {
            CompoundType::Vec(t) => format!("Vec<{}>", t.to_type()),
            CompoundType::Option(t) => format!("Option<{}>", t.to_type()),
            CompoundType::Tuple(t, u) => format!("({}, {})", t.to_type(), u.to_type()),
            CompoundType::Array(t, s) => format!("[{}; {}]", t.to_type(), s),
        }
    }
}

impl ToType for StructField {
    fn to_type(&self) -> String {
        match *self {
            StructField::Type(t) => t.to_type(),
            StructField::CompoundType(t) => t.to_type(),
        }
    }
}

trait ToGenerator {
    fn to_generator(&self) -> String;
}

impl ToGenerator for Type {
    fn to_generator(&self) -> String {
        let generator = match *self {
            Type::String => "rand_string(rng)",
            _ => "rng.gen()",
        };
        generator.to_string()
    }
}

impl ToGenerator for CompoundType {
    fn to_generator(&self) -> String {
        match *self {
            CompoundType::Vec(t) => {
                format!("rand_range(rng, 64).map(|_| {}).collect()",
                        t.to_generator())
            }
            CompoundType::Option(t) => {
                format!("{{ let value = {}; rand_option(rng, value) }}",
                        t.to_generator())
            }
            CompoundType::Tuple(t, u) => format!("({}, {})", t.to_generator(), u.to_generator()),
            CompoundType::Array(t, s) => {
                let parts: Vec<String> = (0..s).map(|_| t.to_generator()).collect();
                format!("[{}]", parts.join(","))
            }
        }
    }
}

impl ToGenerator for StructField {
    fn to_generator(&self) -> String {
        match *self {
            StructField::Type(t) => t.to_generator(),
            StructField::CompoundType(t) => t.to_generator(),
        }
    }
}

impl Rand for Type {
    fn rand<R: Rng>(rng: &mut R) -> Type {
        *rng.choose(&[Type::U8,
                      Type::U16,
                      Type::U32,
                      Type::U64,
                      Type::I8,
                      Type::I16,
                      Type::I32,
                      Type::I64,
                      Type::F32,
                      Type::F64,
                      Type::Char,
                      Type::Bool,
                      Type::String])
             .unwrap()
    }
}

impl Rand for CompoundType {
    fn rand<R: Rng>(rng: &mut R) -> CompoundType {
        match rng.gen::<u64>() % 4 {
            0 => CompoundType::Vec(rng.gen()),
            1 => CompoundType::Option(rng.gen()),
            2 => CompoundType::Tuple(rng.gen(), rng.gen()),
            3 => CompoundType::Array(rng.gen(), rng.gen::<usize>() % 16 + 1),
            _ => unreachable!(),
        }
    }
}

impl Rand for StructField {
    fn rand<R: Rng>(rng: &mut R) -> StructField {
        match rng.gen::<u64>() % 2 {
            0 => StructField::Type(rng.gen()),
            1 => StructField::CompoundType(rng.gen()),
            _ => unreachable!(),
        }
    }
}

impl Rand for Struct {
    fn rand<R: Rng>(rng: &mut R) -> Struct {
        let n = rng.gen_range(1, 16);
        Struct((0..n).map(|_| rng.gen()).collect())
    }
}
