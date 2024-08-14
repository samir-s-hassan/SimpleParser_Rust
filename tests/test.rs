use asalang::*;
use asalang::Node::*;

macro_rules! test {
  ($func:ident, $input:tt, $combinator:tt, $test:expr) => (
    #[test]
    fn $func() -> Result<(),()> {
      let source = $input;
      let tokens = lex(source);
      let parse_result = $combinator(tokens);
      match parse_result {
        Ok((_,tree)) => {
          assert_eq!(tree,$test)
        },
        _ => {assert!(false)},
      }
      Ok(())
    }
  )
}

test!(test_ident, r#"hello"#, identifier, Identifier{value: vec![104, 101, 108, 108, 111]});
test!(test_number, r#"123"#, number, Number{value: vec![49, 50, 51]});
test!(test_bool, r#"true"#, boolean, Bool{value: true});
test!(test_string, r#""hello""#, string, String{value: vec![104, 101, 108, 108, 111]});
test!(test_function_call, r#"foo()"#, function_call, FunctionCall{name: vec![102, 111, 111], children: vec![
  FunctionArguments{ children: vec![
  ]}
]});
test!(test_function_call_one_arg, r#"foo(a)"#, function_call, FunctionCall{name: vec![102, 111, 111], children: vec![
  FunctionArguments{ children: vec![
    Expression { children: vec![Identifier { value: vec![97] }]}
  ]}
]});
test!(test_variable_define_number, r#"let a = 123;"#, variable_define, VariableDefine{children: vec![
  Identifier { value: vec![97] },
  Expression { children: vec![Number{value: vec![49, 50, 51] }]}
]});
test!(test_variable_define_bool, r#"let a = true;"#, variable_define, VariableDefine{children: vec![
  Identifier { value: vec![97] },
  Expression { children: vec![Bool{value: true}]}
]});
test!(test_math_expr, r#"1+1;"#, math_expression, MathExpression {name: vec![97, 100, 100], children: vec![
  Number{value: vec![49]},
  Number{value: vec![49]}
]});
test!(test_variable_define_math_expr, r#"let a = 1 + 1;"#, variable_define, VariableDefine{children: vec![
  Identifier { value: vec![97] },
  Expression { children: vec![
    MathExpression {name: vec![97, 100, 100], children: vec![
      Number{value: vec![49]},
      Number{value: vec![49]}
    ]}
  ]}
]});
test!(test_variable_function_call, r#"let a = foo();"#, variable_define, VariableDefine{children: vec![
  Identifier { value: vec![97] },
  Expression { children: vec![
    FunctionCall{name: vec![102, 111, 111], children: vec![
      FunctionArguments{ children: vec![
      ]}
    ]}
  ]}
]});
test!(test_function_define, r#"fn a(){return 1;}"#, function_define, FunctionDefine{
  name: vec![97],
  children: vec![
    FunctionArguments{ children: vec![] },
    FunctionStatements{ children: vec![
      FunctionReturn{ children: vec![ 
        Expression { children: vec![Number{value: vec![49] }]}
      ]}
    ]}
  ]
});
test!(test_function_define_multi_statements, r#"fn foo(a,b){let x=a+b;return x;}"#, function_define, FunctionDefine{
  name: vec![97, 100, 100],
  children: vec![
    FunctionArguments{ children: vec![] },
    FunctionStatements{ children: vec![
      VariableDefine{children: vec![
        Identifier { value: vec![120] },
        Expression { children: vec![
          MathExpression {name: vec![97, 100, 100], children: vec![
            Identifier{value: vec![97]},
            Identifier{value: vec![98]}
          ]}
        ]}
      ]},
      FunctionReturn{ children: vec![ 
        Expression { children: vec![Identifier{value: vec![120] }]}
      ]}
    ]}
  ]
});
//<-------------------------------AT LEAST 10 MORE TESTS------------------------------->
test!(test_ident_uppercase, r#"HELLO"#, identifier, Identifier{value: vec![72, 69, 76, 76, 79]});
test!(test_number_negative, r#"-123"#, number, Number{value: vec![45, 49, 50, 51]});
test!(test_bool_lowercase, r#"false"#, boolean, Bool{value: false});
test!(test_string_with_spaces, r#""hello world""#, string, String{value: vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]});
test!(test_math_expr_with_parentheses, r#"(1+1)*2;"#, math_expression, MathExpression {name: vec![97, 100, 100], children: vec![
  MathExpression {name: vec![97, 100, 100], children: vec![
    Number{value: vec![49]},
    Number{value: vec![49]}
  ]},
  Number{value: vec![50]}
]});
test!(test_variable_define_boolean_expression, r#"let a = true && false;"#, variable_define, VariableDefine{children: vec![
  Identifier { value: vec![97] },
  Expression { children: vec![
    MathExpression {name: vec![97, 110, 100], children: vec![
      Bool{value: true},
      Bool{value: false}
    ]}
  ]}
]});
test!(test_function_call_multiple_args, r#"foo(a, b, c)"#, function_call, FunctionCall{name: vec![102, 111, 111], children: vec![
  FunctionArguments{ children: vec![
    Expression { children: vec![Identifier { value: vec![97] }]},
    Expression { children: vec![Identifier { value: vec![98] }]},
    Expression { children: vec![Identifier { value: vec![99] }]}
  ]}
]});
test!(test_variable_define_negative_number, r#"let a = -123;"#, variable_define, VariableDefine{children: vec![
  Identifier { value: vec![97] },
  Expression { children: vec![Number{value: vec![45, 49, 50, 51] }]}
]});
test!(test_function_define_empty_args_empty_statements, r#"fn foo(){}"#, function_define, FunctionDefine{
  name: vec![102, 111, 111],
  children: vec![
    FunctionArguments{ children: vec![] },
    FunctionStatements{ children: vec![] }
  ]
});
test!(test_function_call_no_args_lowercase, r#"foo()"#, function_call, FunctionCall{name: vec![102, 111, 111], children: vec![
  FunctionArguments{ children: vec![
  ]}
]});
test!(test_variable_define_true_boolean_uppercase, r#"let a = TRUE;"#, variable_define, VariableDefine{children: vec![
  Identifier { value: vec![97] },
  Expression { children: vec![Bool{value: true}]}
]});
