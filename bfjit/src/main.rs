mod bfir;

fn main() {
    //let result = bfir::compile("fn main[][println1[hellowold]]");
    //println!("{:?}", result);
    //println!("{:?}", bfir::BfIR::GetByte);
    println!("{:?}",bfir::compile("+[,.]"));
}
