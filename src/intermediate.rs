use std::collections::HashMap;

#[derive(Debug)]
pub enum CostantBindFloat{
    BindToValue(FloatShader),
    UnBound, 
}

#[derive(Debug)]
pub enum FloatShader{
    Float(f32),
    Float2(f32,f32),
    Float3(f32,f32,f32),
    Float4(f32,f32,f32,f32),
}

#[derive(Debug)]
pub enum IntShader{
    Float(i32),
    Float2(i32,i32),
    Float3(i32,i32,i32),
    Float4(i32,i32,i32,i32),
}

#[derive(Debug)]
pub enum SemanticsType{
    Position,   //Position doesn't have number
    Color(u8),
    TexCoord(u8),
}

#[derive(Debug)]
pub struct AsmIntermediate{
    model : String,
    semantics_input : HashMap<u8, SemanticsType>,
    float_costants : HashMap<u8,CostantBindFloat>,
    int_costants : HashMap<u8,IntShader>,
    semantics_output : HashMap<u8, String>,
}

#[derive(Debug)]
pub enum RegisterType{
    Input(u8),
    Output3(u8),   //Shader Model 3 have just o#
    Output2(String)      //Shader model 2 or previous have oName
 //   Sampler(u8),
    ConstantF(u8),
} 

impl AsmIntermediate{
    pub fn create_intermediate(model : String) -> AsmIntermediate{
        AsmIntermediate {model : model , semantics_input : HashMap::new(), float_costants : HashMap::new(), int_costants : HashMap::new(), semantics_output : HashMap::new()}
    }
    pub fn add_constants_float(&mut self, reg : u8 , constant :  Option<FloatShader>  ){
        let binded = match constant {
            Some(val) => CostantBindFloat::BindToValue(val),
            None => CostantBindFloat::UnBound,
        };
        
        self.float_costants.insert(reg, binded);
    }
    pub fn add_constants_int(&mut self, cosnt :  IntShader  ){}
    pub fn add_input_semantic(&mut self, reg : u8, constant : SemanticsType){
        self.semantics_input.insert(reg, constant);
    }
    pub fn add_output_sem(&mut self, cosnt :  (u32, String)  ){}

}
