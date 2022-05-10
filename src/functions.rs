use crate::asm::AsmOp;

pub trait Instruction{
    fn parse_to(op : AsmOp);
    fn get_args();
}


pub struct DefInstruction{
    register : u8,
    x : f32,
    y : f32,
    z : f32,
    w : f32, 
} 

 impl Instruction for DefInstruction{
     fn parse_to(op: AsmOp){}
     fn get_args() {}
 }