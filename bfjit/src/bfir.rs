use std::fmt;

#[derive(Debug)]
pub enum BfIR{
    AddVal(u8), // + , AddVal, SubVal 表示将当前单元加减某一数值。
    SubVal(u8), // - , AddPtr, SubPtr 示将当前单元加减某一数值。
    AddPtr(u32), // >, Jz，Jnz 表示跳转指令，带有跳转地址。 
    SubPtr(u32), // <, Jz，Jnz 表示跳转指令，带有跳转地址。
    GetByte, // ,
    PutByte, // .
    Jz, // [
    Jnz, // ]
}

//错误定义
#[derive(Debug, thiserror::Error)]
pub enum CompileErrorKind {
    #[error("unclosed left bracket")]
    UnclosedLeftBracket,
    #[error("Unexpected right bracket")]
    UnexpectedRightBracket,
}

#[derive(Debug)]
pub struct  CompileError {
    line: u32,
    col: u32,
    pub kind: CompileErrorKind,
}

impl std::fmt::Display for CompileError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> std::fmt::Result{
        write!(f, "{} at line {}:{}", self.kind, self.line, self.col)
    }
}

//编译为IR
//compile 函数接收一个字符串，返回 IR 序列。
//code 存储已解析的 IR，stk 作为栈，存储左括号的IR位置、源代码行位置、源代码列位置，line 和 col 分别记录行号和列号。
pub fn compile(src: &str)->Result<Vec<BfIR>, CompileError>{
    let mut code: Vec<BfIR> = vec![];
    let mut stk: Vec<(u32, u32, u32)> = vec![];
    let mut line: u32 = 1;
    let mut col: u32 = 0;

    for ch in src.chars() {
        col += 1;
        match ch {
            '\n' => {//处理换行
                line += 1;
                col = 0;
            },
            '+' => code.push(BfIR::AddVal(1)),
            '-' => code.push(BfIR::SubVal(1)),
            '>' => code.push(BfIR::AddPtr(1)),
            '<' => code.push(BfIR::SubPtr(1)),
            ',' => code.push(BfIR::GetByte),
            '.' => code.push(BfIR::PutByte),
            '[' => {
                let pos = code.len() as u32;
                stk.push((pos, line, col));
                code.push(BfIR::Jz);
            },
            ']' => {
                stk.pop().ok_or(CompileError {
                   line: line,
                   col: col,
                   kind: CompileErrorKind::UnexpectedRightBracket,
                })?;       
                code.push(BfIR::Jnz);
            },
            _=>{},
        }
    }
    //循环结束后，如果栈不为空，说明有左括号没有匹配到右括号，弹出左括号位置，生成编译错误。最后返回生成的IR.
    if let Option::Some((_, line, col)) = stk.pop() {
        return Err(CompileError{
            line: line,
            col: col,
            kind: CompileErrorKind::UnclosedLeftBracket,
        });
    }
    Result::Ok(code)
}

pub fn optimize(code: &mut Vec<BfIR>){
    //代码vec的元素个数
    let len = code.len();
    let mut i = 0;
    let mut pc = 0;
    
    macro_rules! _fold_ir{
        
    }
}
