use std::collections::HashMap;

use byte_string::ByteString;

#[repr(u8)]
pub enum OpCode {
    // Opcodes implementing the basic language
    Call,
    Construct,
    Duplicate,
    Instantiate,
    JumpIf,
    JumpIfNot,
    Load,
    LoadFrom,
    Pop,
    PushGlobal,
    PushLocal,
    PushMember,
    PushNumber,
    PushSelf,
    PushString,
    Return,
    Store,
    StoreKeep,

    // Opcodes implementing standard library functions
    Add,
    Concat,
    Divide,
    Equal,
    Floor,
    GreaterEqual,
    GreaterThan,
    Index,
    InputChar,
    InputEof,
    InputLine,
    Length,
    LessEqual,
    LessThan,
    Modulo,
    Multiply,
    NotEqual,
    NumToString,
    OutputNumber,
    OutputString,
    StringEqual,
    StringReplace,
    StringSplit,
    StringToNum,
    Subtract,
    VarDelete,
    VarNew,
}

pub type LocalName = u16;

pub type MemberName = u16;

pub type GlobalName = u16;

pub type OpcodeIndex = usize;

pub struct ClassDefinition {
    pub funcs: HashMap<MemberName, OpcodeIndex>,

    pub constructor: Option<OpcodeIndex>,
}

impl ClassDefinition {
    pub fn new() -> ClassDefinition {
        ClassDefinition { funcs: HashMap::new(), constructor: None }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FilePosition {
    pub line: usize,

    pub col: usize,
}

pub struct BytecodeProgram {
    pub class_names: Vec<GlobalName>,

    pub classes: Vec<ClassDefinition>,

    pub strings: Vec<ByteString>,

    pub numbers: Vec<f64>,

    pub instructions: Vec<u8>,

    pub main_class: GlobalName,

    pub main_func: MemberName,

    pub files: Vec<(OpcodeIndex, String)>,

    pub positions: Vec<(OpcodeIndex, FilePosition)>,
}
