use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;

use byte_string::ByteString;

use crate::bytecode::*;

const MAIN_CLASS_NAME: &[u8] = b"M";
const MAIN_FUNC_NAME: &[u8] = b"m";
const CONSTRUCTOR_FUNC_NAME: &[u8] = b"c__";

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

impl FilePosition {
    fn advance(&mut self, c: u8) {
        if c == b'\n' {
            self.line += 1;
            self.col = 1;
        }
        else {
            self.col += 1;
        }
    }
}

struct CodeReader<'a> {
    code: Peekable<Iter<'a, u8>>,

    pos: FilePosition,
}

impl<'a> CodeReader<'a> {
    fn new(code: &[u8]) -> CodeReader {
        CodeReader {
            code: code.iter().peekable(),
            pos: FilePosition { line: 1, col: 1 }
        }
    }

    fn peek(&mut self) -> Option<u8> {
        match self.code.peek() {
            Some(c) => Some(**c),
            None => None,
        }
    }

    fn next(&mut self) -> Option<(u8, FilePosition)> {
        match self.code.next() {
            None => None,
            Some(c) => {
                let pos = self.pos;
                self.pos.advance(*c);
                Some((*c, pos))
            }
        }
    }
}

struct BytecodeGenerator {
    instructions: Vec<u8>,

    classes: HashMap<GlobalName, ClassDefinition>,

    member_names: HashMap<ByteString, MemberName>,

    global_names: HashMap<ByteString, GlobalName>,

    local_names: HashMap<ByteString, LocalName>,

    strings: HashMap<ByteString, StringConstantIndex>,

    numbers: Vec<f64>,

    files: Vec<(OpcodeIndex, String)>,

    positions: Vec<(OpcodeIndex, FilePosition)>,
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
            files: Vec::new(),
            positions: Vec::new(),
        }
    }

    fn get_name(name_map: &mut HashMap<ByteString, u16>, name_str: ByteString) -> Option<u16> {
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

    fn get_string_index(&mut self, string: ByteString) -> Option<StringConstantIndex> {
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

    fn get_global_name(&mut self, name_str: ByteString) -> Option<GlobalName> {
        Self::get_name(&mut self.global_names, name_str)
    }

    fn get_member_name(&mut self, name_str: ByteString) -> Option<MemberName> {
        Self::get_name(&mut self.member_names, name_str)
    }

    fn get_local_name(&mut self, name_str: ByteString) -> Option<LocalName> {
        Self::get_name(&mut self.local_names, name_str)
    }

    fn add_func(&mut self, class: &mut ClassDefinition, func_name_str: ByteString) -> Result<(), ParseError> {
        let ByteString(name_bytes) = &func_name_str;
        if name_bytes == CONSTRUCTOR_FUNC_NAME {
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

    fn add_class(&mut self, class: ClassDefinition, class_name_str: ByteString) -> Result<(), ParseError> {
        let class_name = match self.get_global_name(class_name_str.clone()) {
            Some(class_name) => class_name,
            None => return Err(ParseError::TooManyGlobals),
        };

        match self.classes.entry(class_name) {
            Entry::Vacant(entry) => {
                let ByteString(name_bytes) = &class_name_str;
                if name_bytes == MAIN_CLASS_NAME {
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

    fn set_filename(&mut self, file: String) {
        self.files.push((self.instructions.len(), file));
    }

    fn set_position(&mut self, pos: FilePosition) {
        match self.positions.last() {
            Some((_, old_pos)) => {
                if pos != *old_pos {
                    self.positions.push((self.instructions.len(), pos));
                }
            },
            _ => self.positions.push((self.instructions.len(), pos)),
        }
    }

    fn add_opcode(&mut self, opcode: OpCode, pos: FilePosition) {
        self.set_position(pos);
        self.instructions.push(opcode as u8);
    }

    fn add_call(&mut self, pos: FilePosition) {
        self.add_opcode(OpCode::Call, pos);
    }

    fn add_construct(&mut self, pos: FilePosition) {
        self.add_opcode(OpCode::Construct, pos);
    }

    fn add_duplicate(&mut self, index: u8, pos: FilePosition) {
        self.add_opcode(OpCode::Duplicate, pos);
        self.instructions.push(index);
    }

    fn add_instantiate(&mut self, pos: FilePosition) {
        self.add_opcode(OpCode::Instantiate, pos);
    }

    fn add_jump_if(&mut self, loop_start: usize, pos: FilePosition) -> Result<(), ParseError> {
        let jump_amount = self.instructions.len() - loop_start + 3;
        if jump_amount > (u16::MAX as usize) {
            return Err(ParseError::LoopTooLong);
        }

        let hi = (jump_amount >> 8) as u8;
        let lo = (jump_amount & 0xFF) as u8;

        self.add_opcode(OpCode::JumpIf, pos);
        self.instructions.push(hi);
        self.instructions.push(lo);

        self.instructions[loop_start - 2] = hi;
        self.instructions[loop_start - 1] = lo;

        Ok(())
    }

    fn add_jump_if_not(&mut self, pos: FilePosition) -> usize {
        self.add_opcode(OpCode::JumpIfNot, pos);
        self.instructions.push(0);
        self.instructions.push(0);

        self.instructions.len()
    }

    fn add_load(&mut self, pos: FilePosition) {
        self.add_opcode(OpCode::Load, pos);
    }

    fn add_load_from(&mut self, pos: FilePosition) {
        self.add_opcode(OpCode::LoadFrom, pos);
    }

    fn add_pop(&mut self, pos: FilePosition) {
        self.add_opcode(OpCode::Pop, pos);
    }

    fn add_push_global(&mut self, name_str: ByteString, pos: FilePosition) -> Result<(), ParseError> {
        let global_name = match self.get_global_name(name_str) {
            Some(global_name) => global_name,
            None => return Err(ParseError::TooManyGlobals),
        };

        self.add_opcode(OpCode::PushGlobal, pos);
        self.instructions.push((global_name >> 8) as u8);
        self.instructions.push((global_name & 0xFF) as u8);

        Ok(())
    }

    fn add_push_local(&mut self, name_str: ByteString, pos: FilePosition) -> Result<(), ParseError> {
        let local_name = match self.get_local_name(name_str) {
            Some(local_name) => local_name,
            None => return Err(ParseError::TooManyGlobals),
        };

        self.add_opcode(OpCode::PushLocal, pos);
        self.instructions.push((local_name >> 8) as u8);
        self.instructions.push((local_name & 0xFF) as u8);

        Ok(())
    }

    fn add_push_member(&mut self, name_str: ByteString, pos: FilePosition) -> Result<(), ParseError> {
        let member_name = match self.get_member_name(name_str) {
            Some(member_name) => member_name,
            None => return Err(ParseError::TooManyMembers),
        };

        self.add_opcode(OpCode::PushMember, pos);
        self.instructions.push((member_name >> 8) as u8);
        self.instructions.push((member_name & 0xFF) as u8);

        Ok(())
    }

    fn add_push_name(&mut self, name_str: ByteString, pos: FilePosition) -> Result<(), ParseError> {
        match name_str[0] {
            b'A' ..= b'Z' => self.add_push_global(name_str, pos),
            b'a' ..= b'z' => self.add_push_member(name_str, pos),
            b'_' => self.add_push_local(name_str, pos),
            _ => Err(ParseError::UnexpectedName),
        }
    }

    fn add_push_number(&mut self, number: f64, pos: FilePosition) -> Result<(), ParseError> {
        let number_index = match self.get_number_index(number) {
            Some(index) => index,
            None => return Err(ParseError::TooManyNumbers),
        };

        self.add_opcode(OpCode::PushNumber, pos);
        self.instructions.push((number_index >> 8) as u8);
        self.instructions.push((number_index & 0xFF) as u8);

        Ok(())
    }

    fn add_push_self(&mut self, pos: FilePosition) {
        self.add_opcode(OpCode::PushSelf, pos);
    }

    fn add_push_string(&mut self, string: ByteString, pos: FilePosition) -> Result<(), ParseError> {
        let string_index = match self.get_string_index(string) {
            Some(string_index) => string_index,
            None => return Err(ParseError::TooManyMembers),
        };

        self.add_opcode(OpCode::PushString, pos);
        self.instructions.push((string_index >> 8) as u8);
        self.instructions.push((string_index & 0xFF) as u8);

        Ok(())
    }

    fn add_return(&mut self, pos: FilePosition) {
        self.add_opcode(OpCode::Return, pos);
    }

    fn add_store(&mut self, pos: FilePosition) {
        self.add_opcode(OpCode::Store, pos);
    }

    fn add_store_keep(&mut self, pos: FilePosition) {
        self.add_opcode(OpCode::StoreKeep, pos);
    }

    fn get_program(self) -> Result<BytecodeProgram, ParseError> {
        let mut class_names = Vec::new();
        let mut classes = Vec::new();

        for (name, class) in self.classes {
            class_names.push(name);
            classes.push(class);
        }

        let mut strings = std::vec::from_elem(ByteString::new(vec![]), self.strings.len());
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
            files: self.files,
            positions: self.positions,
        })
    }
}

fn add_builtin_classes(gen: &mut BytecodeGenerator) {
    gen.set_filename("<builtin>".to_owned());
    let builtin_pos = FilePosition { line: 0, col: 0 };

    // Arithmetic class
    let mut math = ClassDefinition::new();
    let _ = gen.add_func(&mut math, ByteString::new(vec![b'a']));
    gen.add_opcode(OpCode::Add, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut math, ByteString::new(vec![b'd']));
    gen.add_opcode(OpCode::Divide, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut math, ByteString::new(vec![b'e']));
    gen.add_opcode(OpCode::Equal, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut math, ByteString::new(vec![b'f']));
    gen.add_opcode(OpCode::Floor, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut math, ByteString::new(vec![b'g', b'e']));
    gen.add_opcode(OpCode::GreaterEqual, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut math, ByteString::new(vec![b'g', b't']));
    gen.add_opcode(OpCode::GreaterThan, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut math, ByteString::new(vec![b'l', b'e']));
    gen.add_opcode(OpCode::LessEqual, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut math, ByteString::new(vec![b'l', b't']));
    gen.add_opcode(OpCode::LessThan, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut math, ByteString::new(vec![b'm']));
    gen.add_opcode(OpCode::Multiply, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut math, ByteString::new(vec![b'm', b'o', b'd']));
    gen.add_opcode(OpCode::Modulo, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut math, ByteString::new(vec![b'n', b'e']));
    gen.add_opcode(OpCode::NotEqual, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut math, ByteString::new(vec![b's']));
    gen.add_opcode(OpCode::Subtract, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_class(math, ByteString::new(vec![b'A']));

    // Input class
    let mut input = ClassDefinition::new();
    let _ = gen.add_func(&mut input, ByteString::new(vec![b'c']));
    gen.add_opcode(OpCode::InputChar, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut input, ByteString::new(vec![b'e']));
    gen.add_opcode(OpCode::InputEof, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut input, ByteString::new(vec![b'l']));
    gen.add_opcode(OpCode::InputLine, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_class(input, ByteString(vec![b'I']));

    // Output class
    let mut output = ClassDefinition::new();
    let _ = gen.add_func(&mut output, ByteString::new(vec![b'o']));
    gen.add_opcode(OpCode::OutputString, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut output, ByteString::new(vec![b'o', b'n']));
    gen.add_opcode(OpCode::OutputNumber, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_class(output, ByteString::new(vec![b'O']));

    // String class
    let mut string = ClassDefinition::new();
    let _ = gen.add_func(&mut string, ByteString::new(vec![b'a']));
    gen.add_opcode(OpCode::Concat, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut string, ByteString::new(vec![b'd']));
    gen.add_opcode(OpCode::StringSplit, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut string, ByteString::new(vec![b'e']));
    gen.add_opcode(OpCode::StringEqual, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut string, ByteString::new(vec![b'i']));
    gen.add_opcode(OpCode::Index, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut string, ByteString::new(vec![b'l']));
    gen.add_opcode(OpCode::Length, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut string, ByteString::new(vec![b'n', b's']));
    gen.add_opcode(OpCode::NumToString, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut string, ByteString::new(vec![b's', b'i']));
    gen.add_opcode(OpCode::StringReplace, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut string, ByteString::new(vec![b's', b'n']));
    gen.add_opcode(OpCode::StringToNum, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_class(string, ByteString::new(vec![b'S']));

    // Variable class
    let mut vars = ClassDefinition::new();
    let _ = gen.add_func(&mut vars, ByteString::new(vec![b'd']));
    gen.add_opcode(OpCode::VarDelete, builtin_pos);
    gen.add_return(builtin_pos);
    let _ = gen.add_func(&mut vars, ByteString::new(vec![b'n']));
    gen.add_opcode(OpCode::VarNew, builtin_pos);
    gen.add_return(builtin_pos);

    let _ = gen.add_class(vars, ByteString::new(vec![b'V']));
}

fn skip_whitespace(reader: &mut CodeReader) -> bool {
    while let Some(c) = reader.peek() {
        if c == b'\'' {
            reader.next();
            loop {
                match reader.next() {
                    Some((b'\'', _)) => break,
                    Some(_) => {},
                    None => return false,
                }
            }
            continue;
        }
        else if !c.is_ascii_whitespace() {
            return true;
        }

        reader.next();
    }

    false
}

fn valid_name(name: &ByteString) -> bool {
    if name.len() == 0 {
        return false;
    }

    if name[0].is_ascii_digit() {
        return false;
    }

    for c in name {
        if *c != b'_' && !c.is_ascii_alphanumeric() {
            return false;
        }
    }

    true
}

fn get_integer(int_str: &ByteString) -> Result<u8, ParseError> {
    if int_str.len() == 0 {
        return Err(ParseError::InvalidNumber)
    }

    let mut integer: usize = 0;

    for c in int_str {
        if !c.is_ascii_digit() {
            println!("{}", c);
            return Err(ParseError::InvalidInteger);
        }
        integer = (integer * 10) + (*c as usize - '0' as usize);
        if integer > (u8::MAX as usize) {
            return Err(ParseError::IndexTooBig);
        }
    }

    Ok(integer as u8)
}

fn parse_name(reader: &mut CodeReader) -> Option<ByteString> {
    if !skip_whitespace(reader) {
        return None;
    }

    match reader.peek() {
        Some(c) if c.is_ascii_alphabetic() => {
            match reader.next() {
                Some((c, _)) => Some(ByteString::new(vec![c])),
                _ => unreachable!(),
            }
        },
        Some(b'(') => {
            reader.next();
            let mut name = ByteString::new(vec![]);
            loop {
                match reader.next() {
                    Some((b')', _)) => {
                        if valid_name(&name) {
                            return Some(name);
                        }
                        else {
                            return None;
                        }
                    }
                    Some((c, _)) => name.push(c),
                    None => return None,
                }
            }
        },
        _ => None,
    }
}

fn parse_function(reader: &mut CodeReader, class: &mut ClassDefinition, gen: &mut BytecodeGenerator) -> Result<(), ParseError> {
    assert!(match reader.next() { Some((b'[', _)) => true, _ => false });

    let name = match parse_name(reader) {
        Some(name) => name,
        None => return Err(ParseError::MissingFuncName),
    };

    gen.add_func(class, name)?;

    let mut loop_stack = Vec::new();

    while skip_whitespace(reader) {
        match reader.next() {
            Some((b',', pos)) => gen.add_pop(pos),
            Some((b'^', pos)) => gen.add_return(pos),
            Some((b'*', pos)) => gen.add_load(pos),
            Some((b'=', pos)) => gen.add_store(pos),
            Some((b'?', pos)) => gen.add_call(pos),
            Some((b'.', pos)) => gen.add_load_from(pos),
            Some((c, pos)) if c.is_ascii_lowercase() => gen.add_push_member(ByteString::new(vec![c]), pos)?,
            Some((c, pos)) if c.is_ascii_uppercase() => gen.add_push_global(ByteString::new(vec![c]), pos)?,
            Some((c, pos)) if c.is_ascii_digit() => gen.add_duplicate(c - ('0' as u8), pos),
            Some((b'$', pos)) => {
                gen.add_push_self(pos);
                gen.add_store(pos);
            },
            Some((b'!', pos)) => {
                gen.add_load(pos);
                gen.add_instantiate(pos);
                gen.add_store_keep(pos);
                gen.add_construct(pos);
            },
            Some((b'/', pos)) => {
                let loop_name = match parse_name(reader) {
                    Some(name) => name,
                    None => return Err(ParseError::MissingLoopName),
                };

                gen.add_push_name(loop_name.clone(), pos)?;
                gen.add_load(pos);
                loop_stack.push((loop_name, gen.add_jump_if_not(pos)))
            },
            Some((b'\\', pos)) => {
                match loop_stack.pop() {
                    None => return Err(ParseError::InvalidChar),
                    Some((loop_name, loop_start)) => {
                        gen.add_push_name(loop_name, pos)?;
                        gen.add_load(pos);
                        gen.add_jump_if(loop_start, pos)?;
                    }
                }
            },
            Some((b'(', pos)) => {
                let mut name = ByteString::new(vec![]);
                loop {
                    match reader.next() {
                        Some((b')', _)) => {
                            if valid_name(&name) {
                                gen.add_push_name(name, pos)?;
                                break;
                            }
                            else {
                                gen.add_duplicate(get_integer(&name)?, pos);
                                break;
                            }
                        }
                        Some((c, _)) => name.push(c),
                        None => return Err(ParseError::UnendedParentheses),
                    }
                }
            },
            Some((b'"', pos)) => {
                let mut string = ByteString::new(vec![]);
                loop {
                    match reader.next() {
                        Some((b'"', _)) => {
                            gen.add_push_string(string, pos)?;
                            break;
                        }
                        Some((b'\\', _)) => {
                            match reader.next() {
                                Some((b'n', _)) => string.push(b'\n'),
                                Some((c, _)) => string.push(c),
                                None => return Err(ParseError::UnendedString),
                            }
                        }
                        Some((c, _)) => string.push(c),
                        None => return Err(ParseError::UnendedString),
                    }
                }
            },
            Some((b'<', pos)) => {
                let mut num_str = String::new();
                loop {
                    match reader.next() {
                        Some((b'>', _)) => {
                            let number = match f64::from_str(&num_str) {
                                Ok(num) => num,
                                Err(_) => return Err(ParseError::InvalidNumber),
                            };

                            gen.add_push_number(number, pos)?;
                            break;
                        },
                        Some((c, _)) => num_str.push(c as char),
                        None => return Err(ParseError::UnendedNumber),
                    }
                }
            },
            Some((b']', pos)) => {
                if !loop_stack.is_empty() {
                    return Err(ParseError::UnendedLoop);
                }

                gen.add_return(pos);
                return Ok(());
            },
            Some(_) => return Err(ParseError::InvalidChar),
            None => unreachable!(),
        }
    }

    Err(ParseError::UnendedFunc)
}

fn parse_class(reader: &mut CodeReader, gen: &mut BytecodeGenerator) -> Result<(), ParseError> {
    assert!(match reader.next() { Some((b'{', _)) => true, _ => false });

    let name = match parse_name(reader) {
        Some(name) => name,
        None => return Err(ParseError::MissingClassName),
    };

    let mut class = ClassDefinition::new();

    while skip_whitespace(reader) {
        match reader.peek() {
            Some(b'[') => {
                parse_function(reader, &mut class, gen)?;
            },
            Some(b'}') => {
                reader.next();
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

pub fn parse_program(files: &Vec<(String, Vec<u8>)>) -> Result<BytecodeProgram, ParseError> {
    let mut gen = BytecodeGenerator::new();

    add_builtin_classes(&mut gen);

    for (filename, code) in files {
        let mut reader = CodeReader::new(code);
        gen.set_filename(filename.clone());

        while skip_whitespace(&mut reader) {
            match reader.peek() {
                Some(b'{') => parse_class(&mut reader, &mut gen)?,
                _ => return Err(ParseError::InvalidChar),
            }
        }
    }

    gen.get_program()
}
