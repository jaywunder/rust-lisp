macro_rules! run_program {
    ($program:expr) => {
        let mut stack = eval::types::CallStack::new();
        let parse_result = debug_expr($program);

        println!("{}\n{}\n", "Program:".cyan(), String::from_utf8($program.to_vec()).unwrap());

        println!("{}\n{:?}", "Parse Result:".cyan(), parse_result);

        if let Ok((_a, b)) = parse_result {
            println!("\n{}", "Evaluating:".cyan());
            eval::eval::eval(b, &mut stack);
            println!("\n{}\n{:?}", "Returns:".cyan(), stack.pop_return());
        }

        println!("{}", DASHES);
    };
}

macro_rules! lisp_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (program, expected) = $value;

            let mut stack = eval::types::CallStack::new();
            let parse_result = debug_expr(program);

            if let Ok((_a, b)) = parse_result {
                eval::eval::eval(b, &mut stack);
            }

            assert_eq!(stack.pop_return(), expected);
        }
    )*
    }
}
