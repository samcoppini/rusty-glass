use std::collections::HashMap;

#[repr(u8)]
pub enum OpCode {
    // Opcodes implementing the basic language
    Call,
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

    // Opcodes implementing standard library functions
    Add,
    Equal,
    GreaterEqual,
    GreaterThan,
    LessEqual,
    LessThan,
    NotEqual,
    OutputNumber,
    OutputString,
}

pub type LocalName = u16;

pub type MemberName = u16;

pub type GlobalName = u16;

pub type OpcodeIndex = usize;

pub struct ClassDefinition {
    pub funcs: HashMap<MemberName, OpcodeIndex>,
}

pub struct BytecodeProgram {
    pub class_names: Vec<GlobalName>,

    pub classes: Vec<ClassDefinition>,

    pub strings: Vec<String>,

    pub numbers: Vec<f64>,

    pub instructions: Vec<u8>,

    pub main_class: GlobalName,

    pub main_func: MemberName,
}
