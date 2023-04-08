use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::iter::Peekable;
use std::str::Chars;
use std::str::FromStr;

use ascii::ToAsciiChar;
use ascii::AsciiChar;
use ascii::AsciiString;

use crate::bytecode::*;

const MAIN_CLASS_NAME: &str = "M";
const MAIN_FUNC_NAME: &str = "m";
const CONSTRUCTOR_FUNC_NAME: &str = "c__";

#[derive(Debug)]
pub enum ParseError {
    DuplicateClassName,
    DuplicateFuncName,
    IndexTooBig,
    InvalidChar,
    InvalidInteger,
    InvalidNumber,
    InvalidParentheses,
    InvalidString,
    LoopTooLong,
    MissingClassName,
    MissingFuncName,
    MissingLoopName,
    MissingMainClass,
    MissingMainFunc,
    UnendedClass,
    UnendedFunc,
    UnendedLoop,
    UnendedNumber,
    UnendedParentheses,
    UnendedString,
    UnexpectedName,
    TooManyGlobals,
    TooManyMembers,
    TooManyNumbers,
    TooManyStrings,
}

type NumberConstantIndex = u16;

type StringConstantIndex = u16;

struct BytecodeGenerator {
    instructions: Vec<u8>,

    classes: HashMap<GlobalName, ClassDefinition>,

    member_names: HashMap<String, MemberName>,

    global_names: HashMap<String, GlobalName>,

    local_names: HashMap<String, LocalName>,

    strings: HashMap<AsciiString, StringConstantIndex>,

    numbers: Vec<f64>,
}

impl BytecodeGenerator {
    fn new() -> Self {
        BytecodeGenerator {
            instructions: Vec::new(),
            classes: HashMap::new(),
            member_names: HashMap::new(),
            global_names: HashMap::new(),
            local_names: HashMap::new(),
            strings: HashMap::new(),
            numbers: Vec::new(),
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
                    name_map.insert(name_str, name);
                    Some(name)
                }
            }
        }
    }

    // TODO: we shouldn't have multiple number indices for the same number, but this
    // isn't immediately easy to do because floats don't play nice with maps
    fn get_number_index(&mut self, num: f64) -> Option<NumberConstantIndex> {
        if self.numbers.len() >= StringConstantIndex::MAX as usize {
            None
        }
        else {
            let index = self.numbers.len();
            self.numbers.push(num);
            Some(index as NumberConstantIndex)
        }
    }

    fn get_string_index(&mut self, string: AsciiString) -> Option<StringConstantIndex> {
        match self.strings.get(&string) {
            Some(index) => Some(*index),
            None => {
                if self.strings.len() >= u16::MAX as usize {
                    None
                }
                else {
                    let index = self.strings.len() as u16;
                    self.strings.insert(string, index);
                    Some(index)
                }
            }
        }
    }

    fn get_global_name(&mut self, name_str: String) -> Option<GlobalName> {
        Self::get_name(&mut self.global_names, name_str)
    }

    fn get_member_name(&mut self, name_str: String) -> Option<MemberName> {
        Self::get_name(&mut self.member_names, name_str)
    }

    fn get_local_name(&mut self, name_str: String) -> Option<LocalName> {
        Self::get_name(&mut self.local_names, name_str)
    }

    fn add_func(&mut self, class: &mut ClassDefinition, func_name_str: String) -> Result<(), ParseError> {
        if func_name_str == CONSTRUCTOR_FUNC_NAME {
            class.constructor = Some(self.instructions.len())
        }

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

    fn add_opcode(&mut self, opcode: OpCode) {
        self.instructions.push(opcode as u8);
    }

    fn add_call(&mut self) {
        self.instructions.push(OpCode::Call as u8);
    }

    fn add_construct(&mut self) {
        self.instructions.push(OpCode::Construct as u8);
    }

    fn add_duplicate(&mut self, index: u8) {
        self.instructions.push(OpCode::Duplicate as u8);
        self.instructions.push(index);
    }

    fn add_instantiate(&mut self) {
        self.instructions.push(OpCode::Instantiate as u8);
    }

    fn add_jump_if(&mut self, loop_start: usize) -> Result<(), ParseError> {
        let jump_amount = self.instructions.len() - loop_start + 3;
        if jump_amount > (u16::MAX as usize) {
            return Err(ParseError::LoopTooLong);
        }

        let hi = (jump_amount >> 8) as u8;
        let lo = (jump_amount & 0xFF) as u8;

        self.instructions.push(OpCode::JumpIf as u8);
        self.instructions.push(hi);
        self.instructions.push(lo);

        self.instructions[loop_start - 2] = hi;
        self.instructions[loop_start - 1] = lo;

        Ok(())
    }

    fn add_jump_if_not(&mut self) -> usize {
        self.instructions.push(OpCode::JumpIfNot as u8);
        self.instructions.push(0);
        self.instructions.push(0);

        self.instructions.len()
    }

    fn add_load(&mut self) {
        self.instructions.push(OpCode::Load as u8);
    }

    fn add_load_from(&mut self) {
        self.instructions.push(OpCode::LoadFrom as u8);
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

    fn add_push_local(&mut self, name_str: String) -> Result<(), ParseError> {
        let local_name = match self.get_local_name(name_str) {
            Some(local_name) => local_name,
            None => return Err(ParseError::TooManyGlobals),
        };

        self.instructions.push(OpCode::PushLocal as u8);
        self.instructions.push((local_name >> 8) as u8);
        self.instructions.push((local_name & 0xFF) as u8);

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

    fn add_push_name(&mut self, name_str: String) -> Result<(), ParseError> {
        match name_str.chars().next().unwrap() {
            'A' ..= 'Z' => self.add_push_global(name_str),
            'a' ..= 'z' => self.add_push_member(name_str),
            '_' => self.add_push_local(name_str),
            _ => Err(ParseError::UnexpectedName),
        }
    }

    fn add_push_number(&mut self, number: f64) -> Result<(), ParseError> {
        let number_index = match self.get_number_index(number) {
            Some(index) => index,
            None => return Err(ParseError::TooManyNumbers),
        };

        self.instructions.push(OpCode::PushNumber as u8);
        self.instructions.push((number_index >> 8) as u8);
        self.instructions.push((number_index & 0xFF) as u8);

        Ok(())
    }

    fn add_push_self(&mut self) {
        self.instructions.push(OpCode::PushSelf as u8);
    }

    fn add_push_string(&mut self, string: AsciiString) -> Result<(), ParseError> {
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

    fn add_store_keep(&mut self) {
        self.instructions.push(OpCode::StoreKeep as u8);
    }

    fn get_program(self) -> Result<BytecodeProgram, ParseError> {
        let mut class_names = Vec::new();
        let mut classes = Vec::new();

        for (name, class) in self.classes {
            class_names.push(name);
            classes.push(class);
        }

        let mut strings = std::vec::from_elem(AsciiString::new(), self.strings.len());
        for (string, index) in self.strings {
            strings[index as usize] = string;
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
            numbers: self.numbers,
            instructions: self.instructions,
            main_class: main_class_name,
            main_func: main_func_name,
        })
    }
}

fn add_builtin_classes(gen: &mut BytecodeGenerator) {
    // Arithmetic class
    let mut math = ClassDefinition::new();
    let _ = gen.add_func(&mut math, "a".to_owned());
    gen.add_opcode(OpCode::Add);
    gen.add_return();
    let _ = gen.add_func(&mut math, "d".to_owned());
    gen.add_opcode(OpCode::Divide);
    gen.add_return();
    let _ = gen.add_func(&mut math, "e".to_owned());
    gen.add_opcode(OpCode::Equal);
    gen.add_return();
    let _ = gen.add_func(&mut math, "f".to_owned());
    gen.add_opcode(OpCode::Floor);
    gen.add_return();
    let _ = gen.add_func(&mut math, "ge".to_owned());
    gen.add_opcode(OpCode::GreaterEqual);
    gen.add_return();
    let _ = gen.add_func(&mut math, "gt".to_owned());
    gen.add_opcode(OpCode::GreaterThan);
    gen.add_return();
    let _ = gen.add_func(&mut math, "le".to_owned());
    gen.add_opcode(OpCode::LessEqual);
    gen.add_return();
    let _ = gen.add_func(&mut math, "lt".to_owned());
    gen.add_opcode(OpCode::LessThan);
    gen.add_return();
    let _ = gen.add_func(&mut math, "m".to_owned());
    gen.add_opcode(OpCode::Multiply);
    gen.add_return();
    let _ = gen.add_func(&mut math, "mod".to_owned());
    gen.add_opcode(OpCode::Modulo);
    gen.add_return();
    let _ = gen.add_func(&mut math, "ne".to_owned());
    gen.add_opcode(OpCode::NotEqual);
    gen.add_return();
    let _ = gen.add_func(&mut math, "s".to_owned());
    gen.add_opcode(OpCode::Subtract);
    gen.add_return();
    let _ = gen.add_class(math, "A".to_owned());

    // Output class
    let mut output = ClassDefinition::new();
    let _ = gen.add_func(&mut output, "o".to_owned());
    gen.add_opcode(OpCode::OutputString);
    gen.add_return();
    let _ = gen.add_func(&mut output, "on".to_owned());
    gen.add_opcode(OpCode::OutputNumber);
    gen.add_return();
    let _ = gen.add_class(output, "O".to_owned());

    // String class
    let mut string = ClassDefinition::new();
    let _ = gen.add_func(&mut string, "a".to_owned());
    gen.add_opcode(OpCode::Concat);
    gen.add_return();
    let _ = gen.add_func(&mut string, "e".to_owned());
    gen.add_opcode(OpCode::StringEqual);
    gen.add_return();
    let _ = gen.add_func(&mut string, "i".to_owned());
    gen.add_opcode(OpCode::Index);
    gen.add_return();
    let _ = gen.add_func(&mut string, "l".to_owned());
    gen.add_opcode(OpCode::Length);
    gen.add_return();
    let _ = gen.add_func(&mut string, "ns".to_owned());
    gen.add_opcode(OpCode::NumToString);
    gen.add_return();
    let _ = gen.add_func(&mut string, "sn".to_owned());
    gen.add_opcode(OpCode::StringToNum);
    gen.add_return();
    let _ = gen.add_class(string, "S".to_owned());
}

fn skip_whitespace(iter: &mut Peekable<Chars>) -> bool {
    while let Some(c) = iter.peek() {
        if *c == '\'' {
            iter.next();
            loop {
                match iter.next() {
                    Some('\'') => break,
                    Some(_) => {},
                    None => return false,
                }
            }
            continue;
        }
        else if !c.is_whitespace() {
            return true;
        }

        iter.next();
    }

    false
}

fn valid_name(name: &String) -> bool {
    if name.len() == 0 {
        return false;
    }

    let first_char = name.chars().nth(0).unwrap();
    if first_char.is_ascii_digit() {
        return false;
    }

    for c in name.chars() {
        if c != '_' && !c.is_ascii_alphanumeric() {
            return false;
        }
    }

    true
}

fn get_integer(int_str: &String) -> Result<u8, ParseError> {
    if int_str.len() == 0 {
        return Err(ParseError::InvalidNumber)
    }

    let mut integer: usize = 0;

    for c in int_str.chars() {
        if !c.is_ascii_digit() {
            println!("{}", c);
            return Err(ParseError::InvalidInteger);
        }
        integer = (integer * 10) + (c as usize - '0' as usize);
        if integer > (u8::MAX as usize) {
            return Err(ParseError::IndexTooBig);
        }
    }

    Ok(integer as u8)
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
        Some('(') => {
            iter.next();
            let mut name = String::new();
            loop {
                match iter.next() {
                    Some(')') => {
                        if valid_name(&name) {
                            return Some(name);
                        }
                        else {
                            return None;
                        }
                    }
                    Some(c) => name.push(c),
                    None => return None,
                }
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

    let mut loop_stack = Vec::new();

    while skip_whitespace(iter) {
        match iter.next() {
            Some(',') => gen.add_pop(),
            Some('^') => gen.add_return(),
            Some('*') => gen.add_load(),
            Some('=') => gen.add_store(),
            Some('?') => gen.add_call(),
            Some('.') => gen.add_load_from(),
            Some(c) if c.is_ascii_lowercase() => gen.add_push_member(c.to_string())?,
            Some(c) if c.is_ascii_uppercase() => gen.add_push_global(c.to_string())?,
            Some(c) if c.is_ascii_digit() => gen.add_duplicate((c as u8) - ('0' as u8)),
            Some('$') => {
                gen.add_push_self();
                gen.add_store();
            },
            Some('!') => {
                gen.add_load();
                gen.add_instantiate();
                gen.add_store_keep();
                gen.add_construct();
            },
            Some('/') => {
                let loop_name = match parse_name(iter) {
                    Some(name) => name,
                    None => return Err(ParseError::MissingLoopName),
                };

                gen.add_push_name(loop_name.clone())?;
                gen.add_load();
                loop_stack.push((loop_name, gen.add_jump_if_not()))
            },
            Some('\\') => {
                match loop_stack.pop() {
                    None => return Err(ParseError::InvalidChar),
                    Some((loop_name, loop_start)) => {
                        gen.add_push_name(loop_name)?;
                        gen.add_load();
                        gen.add_jump_if(loop_start)?;
                    }
                }
            },
            Some('(') => {
                let mut name = String::new();
                loop {
                    match iter.next() {
                        Some(')') => {
                            if valid_name(&name) {
                                gen.add_push_name(name)?;
                                break;
                            }
                            else {
                                gen.add_duplicate(get_integer(&name)?);
                                break;
                            }
                        }
                        Some(c) => name.push(c),
                        None => return Err(ParseError::UnendedParentheses),
                    }
                }
            },
            Some('"') => {
                let mut string = AsciiString::new();
                loop {
                    match iter.next() {
                        Some('"') => {
                            gen.add_push_string(string)?;
                            break;
                        }
                        Some('\\') => {
                            match iter.next() {
                                Some('n') => string.push(AsciiChar::LineFeed),
                                Some(c) => match c.to_ascii_char() {
                                    Ok(ch) => string.push(ch),
                                    Err(_) => return Err(ParseError::InvalidString),
                                },
                                None => return Err(ParseError::UnendedString),
                            }
                        }
                        Some(c) => match c.to_ascii_char() {
                            Ok(ch) => string.push(ch),
                            Err(_) => return Err(ParseError::InvalidString),
                        },
                        None => return Err(ParseError::UnendedString),
                    }
                }
            },
            Some('<') => {
                let mut num_str = String::new();
                loop {
                    match iter.next() {
                        Some('>') => {
                            let number = match f64::from_str(&num_str) {
                                Ok(num) => num,
                                Err(_) => return Err(ParseError::InvalidNumber),
                            };

                            gen.add_push_number(number)?;
                            break;
                        },
                        Some(c) => num_str.push(c),
                        None => return Err(ParseError::UnendedNumber),
                    }
                }
            },
            Some(']') => {
                if !loop_stack.is_empty() {
                    return Err(ParseError::UnendedLoop);
                }

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

    let mut class = ClassDefinition::new();

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
