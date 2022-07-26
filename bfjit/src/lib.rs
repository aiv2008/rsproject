pub mod bfir;
#[test]
fn test_compile(){
    assert_eq!(
        bfir::compile("+[,.]").unwrap(),
        vec![
            bfir::BfIR::AddVal(1),
            bfir::BfIR::Jz,
            bfir::BfIR::GetByte,
            bfir::BfIR::PutByte,
            bfir::BfIR::Jnz,
        ]
    );

    match bfir::compile("[").unwrap_err().kind {
        bfir::CompileErrorKind::UnclosedLeftBracket => {}
        _ => panic!(),
    };

    match bfir::compile("]").unwrap_err().kind {
        bfir::CompileErrorKind::UnexpectedRightBracket => {}
        _ => panic!(),
    };

    let mut code = bfir::compile("[+++++]").unwrap();
    //optimize(&mut code);
    assert_eq!(code, vec![bfir::BfIR::Jz, bfir::BfIR::AddVal(5),bfir::BfIR::Jnz]);
}

