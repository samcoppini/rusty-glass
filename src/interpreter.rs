use std::collections::HashMap;

use crate::bytecode::*;

type ClassIndex = usize;

type StringIndex = usize;

type InstanceIndex = usize;

const OPCODE_ADD: u8 = OpCode::Add as u8;
const OPCODE_CALL: u8 = OpCode::Call as u8;
const OPCODE_DIVIDE: u8 = OpCode::Divide as u8;
const OPCODE_EQUAL: u8 = OpCode::Equal as u8;
const OPCODE_FLOOR: u8 = OpCode::Floor as u8;
const OPCODE_GREATER: u8 = OpCode::GreaterThan as u8;
const OPCODE_GREATER_EQUAL: u8 = OpCode::GreaterEqual as u8;
const OPCODE_INSTANTIATE: u8 = OpCode::Instantiate as u8;
const OPCODE_JUMP_IF: u8 = OpCode::JumpIf as u8;
const OPCODE_JUMP_IF_NOT: u8 = OpCode::JumpIfNot as u8;
const OPCODE_LESS: u8 = OpCode::LessThan as u8;
const OPCODE_LESS_EQUAL: u8 = OpCode::LessEqual as u8;
const OPCODE_LOAD: u8 = OpCode::Load as u8;
const OPCODE_LOAD_FROM: u8 = OpCode::LoadFrom as u8;
const OPCODE_MODULO: u8 = OpCode::Modulo as u8;
const OPCODE_NOT_EQUAL: u8 = OpCode::NotEqual as u8;
const OPCODE_OUTPUT_NUMBER: u8 = OpCode::OutputNumber as u8;
const OPCODE_OUTPUT_STRING: u8 = OpCode::OutputString as u8;
const OPCODE_POP: u8 = OpCode::Pop as u8;
const OPCODE_PUSH_LOCAL: u8 = OpCode::PushLocal as u8;
const OPCODE_PUSH_NUMBER: u8 = OpCode::PushNumber as u8;
const OPCODE_PUSH_MEMBER: u8 = OpCode::PushMember as u8;
const OPCODE_PUSH_GLOBAL: u8 = OpCode::PushGlobal as u8;
const OPCODE_PUSH_SELF: u8 = OpCode::PushSelf as u8;
const OPCODE_PUSH_STRING: u8 = OpCode::PushString as u8;
const OPCODE_RETURN: u8 = OpCode::Return as u8;
const OPCODE_STORE: u8 = OpCode::Store as u8;
const OPCODE_SUBTRACT: u8 = OpCode::Subtract as u8;

#[derive(Clone, Copy)]
enum GlassValue {
    Class(ClassIndex),
    Function(InstanceIndex, OpcodeIndex),
    GlobalName(GlobalName),
    Instance(InstanceIndex),
    LocalName(LocalName),
    MemberName(MemberName),
    Number(f64),
    String(StringIndex),
}

struct GlassInstance<'a> {
    class: &'a ClassDefinition,

    variables: HashMap<MemberName, GlassValue>,
}

#[derive(Debug)]
pub enum RuntimeError {
    EmptyStack,
    UnsetName,
    WrongType,
}

fn instantiate<'a>(instances: &mut Vec<GlassInstance<'a>>, class: &'a ClassDefinition) -> InstanceIndex {
    instances.push(GlassInstance {
        class: &class,
        variables: HashMap::new(),
    });

    instances.len() - 1
}

fn read_short(instructions: &Vec<u8>, index: &mut usize) -> u16 {
    let hi_byte = instructions[*index + 1] as u16;
    let lo_byte = instructions[*index + 2] as u16;
    *index += 2;
    (hi_byte << 8) | lo_byte
}

fn pop_number(value_stack: &mut Vec<GlassValue>) -> Result<f64, RuntimeError> {
    match value_stack.pop() {
        Some(GlassValue::Number(num)) => Ok(num),
        Some(_) => return Err(RuntimeError::WrongType),
        None => return Err(RuntimeError::EmptyStack),
    }
}

pub fn execute_program(program: &BytecodeProgram) -> Result<(), RuntimeError> {
    let mut instances = Vec::new();
    let mut strings = Vec::new();
    let mut func_stack = Vec::new();
    let mut value_stack = Vec::new();
    let mut globals = HashMap::new();
    let mut cur_object = 0 as InstanceIndex;
    let mut locals = HashMap::new();

    // Populate globals with class definitions
    for i in 0..program.classes.len() {
        let class_name = program.class_names[i];
        globals.insert(program.class_names[i], GlassValue::Class(i as ClassIndex));

        if class_name == program.main_class {
            instantiate(&mut instances, &program.classes[i]);
        }
    }

    // Fill out the global string array
    for string in program.strings.iter() {
        strings.push(string.clone());
    }

    let mut opcode_index = instances[cur_object as usize].class.funcs[&program.main_func] as usize;

    loop {
        match program.instructions[opcode_index] {
            OPCODE_ADD => {
                let num1 = pop_number(&mut value_stack)?;
                let num2 = pop_number(&mut value_stack)?;
                value_stack.push(GlassValue::Number(num1 + num2));
            },
            OPCODE_CALL => {
                match value_stack.pop() {
                    Some(GlassValue::Function(call_inst, call_op)) => {
                        func_stack.push((cur_object, opcode_index, locals));
                        locals = HashMap::new();
                        cur_object = call_inst;
                        opcode_index = call_op as usize;
                        continue;
                    },
                    Some(_) => return Err(RuntimeError::WrongType),
                    None => return Err(RuntimeError::EmptyStack),
                }
            },
            OPCODE_DIVIDE => {
                let num1 = pop_number(&mut value_stack)?;
                let num2 = pop_number(&mut value_stack)?;
                value_stack.push(GlassValue::Number(num2 / num1));
            },
            OPCODE_EQUAL => {
                let num1 = pop_number(&mut value_stack)?;
                let num2 = pop_number(&mut value_stack)?;
                value_stack.push(GlassValue::Number(if num1 == num2 { 1.0 } else { 0.0 }));
            },
            OPCODE_FLOOR => {
                let num = pop_number(&mut value_stack)?;
                value_stack.push(GlassValue::Number(num.floor()));
            },
            OPCODE_GREATER => {
                let num1 = pop_number(&mut value_stack)?;
                let num2 = pop_number(&mut value_stack)?;
                value_stack.push(GlassValue::Number(if num1 < num2 { 1.0 } else { 0.0 }));
            },
            OPCODE_GREATER_EQUAL => {
                let num1 = pop_number(&mut value_stack)?;
                let num2 = pop_number(&mut value_stack)?;
                value_stack.push(GlassValue::Number(if num1 <= num2 { 1.0 } else { 0.0 }));
            },
            OPCODE_INSTANTIATE => {
                match value_stack.pop() {
                    Some(GlassValue::Class(class_index)) => {
                        let inst_index = instantiate(&mut instances, &program.classes[class_index as usize]);
                        value_stack.push(GlassValue::Instance(inst_index));
                    },
                    Some(_) => return Err(RuntimeError::WrongType),
                    None => return Err(RuntimeError::EmptyStack),
                }
            },
            OPCODE_JUMP_IF => {
                let should_jump = match value_stack.pop() {
                    Some(GlassValue::Number(num)) => num != 0.0,
                    Some(GlassValue::String(index)) => !strings[index].is_empty(),
                    Some(_) => false,
                    None => return Err(RuntimeError::EmptyStack),
                };

                let jump_amount = read_short(&program.instructions, &mut opcode_index);
                if should_jump {
                    opcode_index -= jump_amount as usize;
                }
            },
            OPCODE_JUMP_IF_NOT => {
                let should_jump = match value_stack.pop() {
                    Some(GlassValue::Number(num)) => num == 0.0,
                    Some(GlassValue::String(index)) => strings[index].is_empty(),
                    Some(_) => true,
                    None => return Err(RuntimeError::EmptyStack),
                };

                let jump_amount = read_short(&program.instructions, &mut opcode_index);
                if should_jump {
                    opcode_index += jump_amount as usize;
                }
            },
            OPCODE_LESS => {
                let num1 = pop_number(&mut value_stack)?;
                let num2 = pop_number(&mut value_stack)?;
                value_stack.push(GlassValue::Number(if num1 > num2 { 1.0 } else { 0.0 }));
            },
            OPCODE_LESS_EQUAL => {
                let num1 = pop_number(&mut value_stack)?;
                let num2 = pop_number(&mut value_stack)?;
                value_stack.push(GlassValue::Number(if num1 >= num2 { 1.0 } else { 0.0 }));
            },
            OPCODE_LOAD => {
                match value_stack.pop() {
                    Some(GlassValue::GlobalName(global_index)) => {
                        match globals.get(&global_index) {
                            Some(val) => value_stack.push(*val),
                            None => return Err(RuntimeError::UnsetName),
                        }
                    },
                    Some(GlassValue::LocalName(local_index)) => {
                        match locals.get(&local_index) {
                            Some(val) => value_stack.push(*val),
                            None => return Err(RuntimeError::UnsetName),
                        }
                    },
                    Some(GlassValue::MemberName(member_index)) => {
                        let instance = &instances[cur_object];
                        match instance.variables.get(&member_index) {
                            Some(val) => value_stack.push(*val),
                            None => return Err(RuntimeError::UnsetName),
                        }
                    },
                    Some(_) => return Err(RuntimeError::WrongType),
                    None => return Err(RuntimeError::EmptyStack),
                }
            },
            OPCODE_LOAD_FROM => {
                let name = match value_stack.pop() {
                    Some(GlassValue::MemberName(name)) => name,
                    Some(_) => return Err(RuntimeError::WrongType),
                    None => return Err(RuntimeError::EmptyStack),
                };

                // TODO: DRY this up. The logic here is the same as the OPCODE_LOAD case
                let loaded_value = match value_stack.pop() {
                    Some(GlassValue::GlobalName(global_index)) => {
                        match globals.get(&global_index) {
                            Some(val) => val,
                            None => return Err(RuntimeError::UnsetName),
                        }
                    },
                    Some(GlassValue::LocalName(local_index)) => {
                        match locals.get(&local_index) {
                            Some(val) => val,
                            None => return Err(RuntimeError::UnsetName),
                        }
                    },
                    Some(GlassValue::MemberName(member_index)) => {
                        let instance = &instances[cur_object];
                        match instance.variables.get(&member_index) {
                            Some(val) => val,
                            None => return Err(RuntimeError::UnsetName),
                        }
                    },
                    Some(_) => return Err(RuntimeError::WrongType),
                    None => return Err(RuntimeError::EmptyStack),
                };

                let inst_index = match loaded_value {
                    GlassValue::Instance(inst_index) => inst_index,
                    _ => return Err(RuntimeError::WrongType),
                };

                let instance = &instances[*inst_index as usize];

                match instance.variables.get(&name) {
                    Some(val) => value_stack.push(*val),
                    None => {
                        match instance.class.funcs.get(&name) {
                            Some(op_index) => value_stack.push(GlassValue::Function(*inst_index, *op_index)),
                            None => return Err(RuntimeError::UnsetName),
                        }
                    },
                }
            },
            OPCODE_MODULO => {
                let num1 = pop_number(&mut value_stack)?;
                let num2 = pop_number(&mut value_stack)?;
                value_stack.push(GlassValue::Number(num2 % num1));
            },
            OPCODE_NOT_EQUAL => {
                let num1 = pop_number(&mut value_stack)?;
                let num2 = pop_number(&mut value_stack)?;
                value_stack.push(GlassValue::Number(if num1 != num2 { 1.0 } else { 0.0 }));
            },
            OPCODE_OUTPUT_NUMBER => {
                match value_stack.pop() {
                    Some(GlassValue::Number(num)) => {
                        print!("{}", num);
                    },
                    Some(_) => return Err(RuntimeError::WrongType),
                    None => return Err(RuntimeError::EmptyStack),
                }
            },
            OPCODE_OUTPUT_STRING => {
                match value_stack.pop() {
                    Some(GlassValue::String(str_index)) => {
                        print!("{}", strings[str_index]);
                    },
                    Some(_) => return Err(RuntimeError::WrongType),
                    None => return Err(RuntimeError::EmptyStack),
                }
            },
            OPCODE_POP => {
                if let None = value_stack.pop() {
                    return Err(RuntimeError::EmptyStack);
                }
            },
            OPCODE_PUSH_GLOBAL => {
                let name = read_short(&program.instructions, &mut opcode_index);
                value_stack.push(GlassValue::GlobalName(name as GlobalName));
            },
            OPCODE_PUSH_LOCAL => {
                let name = read_short(&program.instructions, &mut opcode_index);
                value_stack.push(GlassValue::LocalName(name as LocalName));
            },
            OPCODE_PUSH_MEMBER => {
                let name = read_short(&program.instructions, &mut opcode_index);
                value_stack.push(GlassValue::MemberName(name as MemberName));
            },
            OPCODE_PUSH_NUMBER => {
                let num_index = read_short(&program.instructions, &mut opcode_index);
                value_stack.push(GlassValue::Number(program.numbers[num_index as usize]));
            },
            OPCODE_PUSH_SELF => {
                value_stack.push(GlassValue::Instance(cur_object));
            },
            OPCODE_PUSH_STRING => {
                let str_index = read_short(&program.instructions, &mut opcode_index);
                value_stack.push(GlassValue::String(str_index as StringIndex));
            },
            OPCODE_RETURN => {
                match func_stack.pop() {
                    Some((call_inst, call_op, local_vars)) => {
                        cur_object = call_inst;
                        opcode_index = call_op;
                        locals = local_vars;
                    },
                    None => {
                        return Ok(());
                    },
                }
            },
            OPCODE_STORE => {
                let value = match value_stack.pop() {
                    Some(val) => val,
                    None => return Err(RuntimeError::EmptyStack),
                };

                match value_stack.pop() {
                    Some(GlassValue::GlobalName(name)) => {
                        globals.insert(name, value);
                    },
                    Some(GlassValue::LocalName(name)) => {
                        locals.insert(name, value);
                    },
                    Some(GlassValue::MemberName(name)) => {
                        instances[cur_object as usize].variables.insert(name, value);
                    },
                    Some(_) => return Err(RuntimeError::WrongType),
                    None => return Err(RuntimeError::EmptyStack),
                }
            },
            OPCODE_SUBTRACT => {
                let num1 = pop_number(&mut value_stack)?;
                let num2 = pop_number(&mut value_stack)?;
                value_stack.push(GlassValue::Number(num2 - num1));
            },
            _ => unreachable!(),
        }

        opcode_index += 1;
    }
}
