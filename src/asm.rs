use crate::intermediate::AsmIntermediate;
use crate::parser::{ParseResult,parse_instruction};

#[derive(Debug)]
pub struct AsmOp {
    pub instruction : String,
    pub operands : Option<String>
}

impl AsmOp {
    pub fn parse(input : String) -> AsmOp {
        let istr = input.split_once(" ");
        match istr{
            Some((istruction,operands)) => {
               AsmOp {instruction : istruction.to_string(), operands : Some(operands.to_owned())}
            },
            None => {
                AsmOp {instruction : input, operands : None}
            }
        }
    }
}

#[derive(Debug)]
pub struct FileAsm {
    model : String,
    operations : Vec<AsmOp>
}

impl FileAsm {
    pub fn new(file : String) -> FileAsm {   

        let mut operands = Vec::new();
        let mut started_code = false;
        let mut vers = None;
        for line  in file.lines(){
            let line = line.trim();
            if line.chars().nth(0) == Some('\u{0}') || line.is_empty() {
                continue;
            }
            if started_code && !line.starts_with("//"){
                let istr = AsmOp::parse(line.to_owned());
                operands.push(istr);
            }

            if started_code == false && (line.starts_with("vs_") || line.starts_with("ps_")){
                started_code = true;
                vers = Some(line);
            }
        }
        FileAsm {model : vers.unwrap().to_owned(), operations : operands}
    }

    pub fn decompile_step(self) -> AsmIntermediate{
        let mut intermediate = AsmIntermediate::create_intermediate(self.model);
        for op in self.operations{
            let parsed = parse_instruction(&op);
            match parsed {
                ParseResult::ConstantF(reg, val) => {
                    intermediate.add_constants_float(reg,Some(val));
                },
                ParseResult::Semantic(reg, val) => {
                    intermediate.add_input_semantic(reg,val);
                },
                ParseResult::Unknow => println!("Unimplemented {:?}", op),
                _ => println!("Unimplemented {:?}", op),
            }
        }
        intermediate
    }
}
 
