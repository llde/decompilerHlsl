use crate::asm::AsmOp;
use crate::functions::Instruction;
use crate::intermediate::{FloatShader,SemanticsType};
use std::str::FromStr;
use std::fmt::Debug;

pub enum ParseResult {
   ConstantF(u8, FloatShader),
   ConstantI,
   Semantic(u8, SemanticsType),
   Instr, 
   Unknow,
}

fn parse_unwrap<T : FromStr>(p : &str) -> T where <T as FromStr>::Err: Debug {
    p.parse::<T>().unwrap()
} 

fn get_input_semantic(semantic : &str) -> Option<SemanticsType>{
    let last_char = semantic.chars().last().unwrap();  //FIXME 10 or 11 may be valid?
    let specific = if last_char.is_ascii_digit(){
        parse_unwrap::<u8>(&last_char.to_string())
    } else { parse_unwrap::<u8>(&'0'.to_string()) };
    
    if semantic.starts_with("color"){
        Some(SemanticsType::Color(specific))
    }
    else if semantic.starts_with("texcoord"){
        Some(SemanticsType::TexCoord(specific))
    }
    else if semantic.starts_with("position"){
        Some(SemanticsType::Position)
    }
    else {
        println!("unhandled semantic: {}", semantic);
        return None;
    }
}

fn check_for_register_semantic(register : &str){
    
}

pub fn parse_instruction(op : &AsmOp) -> ParseResult{
    let op_ir = op.instruction.trim();
    let op_spl : Vec<&str> = op_ir.split('_').map(|x| x.trim()).collect();
    let op_i = op_spl[0];
    let opps = op.operands.as_ref();  
    let ops : Vec<&str> = match opps {
        Some(val) => val.split(',').map(|s| s.trim()).collect(),
        None => Vec::new()
    };
    match op_i {
        "def" => {
            let register = ops[0].strip_prefix('c').unwrap().parse::<u8>().unwrap(); //float costants are c registers
            let float = FloatShader::Float4(parse_unwrap::<f32>(ops[1]), parse_unwrap::<f32>(ops[2]),parse_unwrap::<f32>(ops[3]),parse_unwrap::<f32>(ops[4]));
            ParseResult::ConstantF(register, float )
        },
        "dcl" => { 
            let split : Vec<&str> = ops[0].split('.').collect();
            if split.len() > 1 {
                println!("Unhandled Swizzle : {} ", split[1]);
            }
            let register = split[0].strip_prefix('v').unwrap().parse::<u8>().unwrap(); //input semantics are v registers. FIXME swizzle mask
            let semantic = get_input_semantic(op_spl[1]).unwrap();
            //FIXME modifier
            if op_spl.len() > 2 {
                println!("Unhandled modifier: {}", op_spl[2]);
            }
            ParseResult::Semantic(register, semantic)
            
        },
         _ => {
             println!("Can't decompile {}" , op_i);
             ParseResult::Unknow
         },   
    }
}
 
