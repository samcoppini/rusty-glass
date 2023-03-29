use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::iter::Peekable;
use std::str::Chars;

use crate::bytecode::*;

const MAIN_CLASS_NAME: &str = "M";
const MAIN_FUNC_NAME: &str = "m";

#[derive(Debug)]
pub enum ParseError {
    DuplicateClassName,
    DuplicateFuncName,
    InvalidChar,
    MissingClassName,
    MissingFuncName,
    MissingMainClass,
    MissingMainFunc,
    UnendedClass,
    UnendedFunc,
    UnendedString,
    TooManyGlobals,
    TooManyMembers,
    TooManyStrings,
}

type StringConstantIndex = u16;

struct BytecodeGenerator {
    instructions: Vec<u8>,

    classes: HashMap<GlobalName, ClassDefinition>,

    member_names: HashMap<String, MemberName>,

    global_names: HashMap<String, GlobalName>,

    strings: HashMap<String, StringConstantIndex>,
}

impl BytecodeGenerator {
    fn new() -> Self {
        BytecodeGenerator {
            instructions: Vec::new(),
            classes: HashMap::new(),
            member_names: HashMap::new(),
            global_names: HashMap::new(),
            strings: HashMap::new(),
        }
    }

    fn get_name(name_map: &mut HashMap<String, u16>, name_str: String) -> Option<u16> {
        match name_map.get(&name_str) {
            Some(name) => Some(*name),
            None => {
                if name_map.len() >= u16::MAX as usize {
                    None
                }
                else {
                    let name = name_map.len() as u16;
                    name_map.insert(name_str.clone(), name);
                    Some(name)
                }
            }
        }
    }

    fn get_string_index(&mut self, string: String) -> Option<StringConstantIndex> {
        Self::get_name(&mut self.strings, string)
    }

    fn get_global_name(&mut self, name_str: String) -> Option<GlobalName> {
        Self::get_name(&mut self.global_names, name_str)
    }

    fn get_member_name(&mut self, name_str: String) -> Option<MemberName> {
        Self::get_name(&mut self.member_names, name_str)
    }

    fn add_func(&mut self, class: &mut ClassDefinition, func_name_str: String) -> Result<(), ParseError> {
        let member_name = match self.get_member_name(func_name_str) {
            Some(member_name) => member_name,
            None => return Err(ParseError::TooManyMembers),
        };

        match class.funcs.entry(member_name) {
            Entry::Vacant(entry) => {
                entry.insert(self.instructions.len());
                Ok(())
            },
            Entry::Occupied(_) => Err(ParseError::DuplicateFuncName),
        }
    }

    fn add_class(&mut self, class: ClassDefinition, class_name_str: String) -> Result<(), ParseError> {
        let class_name = match self.get_global_name(class_name_str.clone()) {
            Some(class_name) => class_name,
            None => return Err(ParseError::TooManyGlobals),
        };

        match self.classes.entry(class_name) {
            Entry::Vacant(entry) => {
                if class_name_str == MAIN_CLASS_NAME {
                    match self.member_names.get(MAIN_FUNC_NAME) {
                        Some(main_func_name) => {
                            if !class.funcs.contains_key(main_func_name) {
                                return Err(ParseError::MissingMainFunc);
                            }
                        },
                        None => return Err(ParseError::MissingMainFunc),
                    }
                }

                entry.insert(class);
                Ok(())
            },
            Entry::Occupied(_) => Err(ParseError::DuplicateClassName),
        }
    }

    fn add_call(&mut self) {
        self.instructions.push(OpCode::Call as u8);
    }

    fn add_instantiate(&mut self) {
        self.instructions.push(OpCode::Instantiate as u8);
    }

    fn add_load(&mut self) {
        self.instructions.push(OpCode::Load as u8);
    }

    fn add_load_from(&mut self) {
        self.instructions.push(OpCode::LoadFrom as u8);
    }

    fn add_output_string(&mut self) {
        self.instructions.push(OpCode::OutputString as u8);
    }

    fn add_pop(&mut self) {
        self.instructions.push(OpCode::Pop as u8);
    }

    fn add_push_global(&mut self, name_str: String) -> Result<(), ParseError> {
        let global_name = match self.get_global_name(name_str) {
            Some(global_name) => global_name,
            None => return Err(ParseError::TooManyGlobals),
        };

        self.instructions.push(OpCode::PushGlobal as u8);
        self.instructions.push((global_name >> 8) as u8);
        self.instructions.push((global_name & 0xFF) as u8);

        Ok(())
    }

    fn add_push_member(&mut self, name_str: String) -> Result<(), ParseError> {
        let member_name = match self.get_member_name(name_str) {
            Some(member_name) => member_name,
            None => return Err(ParseError::TooManyMembers),
        };

        self.instructions.push(OpCode::PushMember as u8);
        self.instructions.push((member_name >> 8) as u8);
        self.instructions.push((member_name & 0xFF) as u8);

        Ok(())
    }

    fn add_push_string(&mut self, string: String) -> Result<(), ParseError> {
        let string_index = match self.get_string_index(string) {
            Some(string_index) => string_index,
            None => return Err(ParseError::TooManyMembers),
        };

        self.instructions.push(OpCode::PushString as u8);
        self.instructions.push((string_index >> 8) as u8);
        self.instructions.push((string_index & 0xFF) as u8);

        Ok(())
    }

    fn add_return(&mut self) {
        self.instructions.push(OpCode::Return as u8);
    }

    fn add_store(&mut self) {
        self.instructions.push(OpCode::Store as u8);
    }

    fn get_program(self) -> Result<BytecodeProgram, ParseError> {
        let mut class_names = Vec::new();
        let mut classes = Vec::new();

        for (name, class) in self.classes {
            class_names.push(name);
            classes.push(class);
        }

        let mut strings = std::vec::from_elem("".to_owned(), self.strings.len());
        for (string, index) in self.strings {
            strings[index as usize] = string.to_owned();
        }

        let main_class_name = match self.global_names.get(MAIN_CLASS_NAME) {
            Some(name) => *name,
            None => return Err(ParseError::MissingMainClass),
        };

        let main_func_name = match self.member_names.get(MAIN_FUNC_NAME) {
            Some(name) => *name,
            // This should be unreachable...
            None => return Err(ParseError::MissingMainFunc),
        };

        Ok(BytecodeProgram {
            class_names: class_names,
            classes: classes,
            strings: strings,
            instructions: self.instructions,
            main_class: main_class_name,
            main_func: main_func_name,
        })
    }
}

fn add_builtin_classes(gen: &mut BytecodeGenerator) {
    let mut output = ClassDefinition { funcs: HashMap::new() };

    let _ = gen.add_func(&mut output, "o".to_owned());
    gen.add_output_string();
    gen.add_return();

    let _ = gen.add_class(output, "O".to_owned());
}

fn skip_whitespace(iter: &mut Peekable<Chars>) -> bool {
    while let Some(c) = iter.peek() {
        if !c.is_whitespace() {
            return true;
        }

        iter.next();
    }

    false
}

fn parse_name(iter: &mut Peekable<Chars>) -> Option<String> {
    if !skip_whitespace(iter) {
        return None;
    }

    match iter.peek() {
        Some(c) if c.is_ascii_alphabetic() => {
            match iter.next() {
                Some(c) => Some(c.to_string()),
                _ => unreachable!(),
            }
        },
        _ => None,
    }
}

fn parse_function(iter: &mut Peekable<Chars>, class: &mut ClassDefinition, gen: &mut BytecodeGenerator) -> Result<(), ParseError> {
    assert!(match iter.next() { Some('[') => true, _ => false });

    let name = match parse_name(iter) {
        Some(name) => name,
        None => return Err(ParseError::MissingFuncName),
    };

    gen.add_func(class, name)?;

    while skip_whitespace(iter) {
        match iter.next() {
            Some(',') => gen.add_pop(),
            Some('?') => gen.add_call(),
            Some('.') => gen.add_load_from(),
            Some(c) if c.is_ascii_lowercase() => gen.add_push_member(c.to_string())?,
            Some(c) if c.is_ascii_uppercase() => gen.add_push_global(c.to_string())?,
            Some('!') => {
                gen.add_load();
                gen.add_instantiate();
                gen.add_store();
            },
            Some('"') => {
                let mut string = String::new();
                loop {
                    match iter.next() {
                        Some('"') => {
                            gen.add_push_string(string)?;
                            break;
                        }
                        Some(c) => string.push(c),
                        None => return Err(ParseError::UnendedString),
                    }
                }
            },
            Some(']') => {
                gen.add_return();
                return Ok(());
            },
            Some(_) => return Err(ParseError::InvalidChar),
            None => unreachable!(),
        }
    }

    Err(ParseError::UnendedFunc)
}

fn parse_class(iter: &mut Peekable<Chars>, gen: &mut BytecodeGenerator) -> Result<(), ParseError> {
    assert!(match iter.next() { Some('{') => true, _ => false });

    let name = match parse_name(iter) {
        Some(name) => name,
        None => return Err(ParseError::MissingClassName),
    };

    let mut class = ClassDefinition { funcs: HashMap::new() };

    while skip_whitespace(iter) {
        match iter.peek() {
            Some('[') => {
                parse_function(iter, &mut class, gen)?;
            },
            Some('}') => {
                iter.next();
                return gen.add_class(class, name);
            },
            Some(_) => {
                return Err(ParseError::InvalidChar);
            },
            None => unreachable!(),
        }
    }

    Err(ParseError::UnendedClass)
}

pub fn parse_program(code: &str) -> Result<BytecodeProgram, ParseError> {
    let mut gen = BytecodeGenerator::new();
    let mut iter = code.chars().peekable();

    add_builtin_classes(&mut gen);

    while skip_whitespace(&mut iter) {
        match iter.peek() {
            Some('{') => parse_class(&mut iter, &mut gen)?,
            _ => return Err(ParseError::InvalidChar),
        }
    }

    gen.get_program()
}
