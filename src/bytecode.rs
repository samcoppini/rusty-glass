use std::collections::HashMap;

#[repr(u8)]
pub enum OpCode {
    // Opcodes implementing the basic language
    Call,
    Instantiate,
    Load,
    LoadFrom,
    Pop,
    PushGlobal,
    PushMember,
    PushSelf,
    PushString,
    Return,
    Store,

    // Opcodes implementing standard library functions
    OutputString,
}

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

    pub instructions: Vec<u8>,

    pub main_class: GlobalName,

    pub main_func: MemberName,
}
