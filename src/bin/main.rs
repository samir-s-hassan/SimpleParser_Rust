extern crate nom;
extern crate asalang;

use asalang::*;

fn main() -> Result<(), nom::Err<(&'static str, nom::error::ErrorKind)>> {

  let source = "a";
  let tokens = lex(source);
  let parse_result = identifier(tokens);
  println!("{:?}", parse_result);
  match parse_result {
    Ok((_,tree)) => {
      let expected = Node::Identifier{value: vec![104,101,108,108,111]};
      
      println!("{:#?}", tree == expected);
      assert!(true)
    },
    _ => {assert!(false)},
  }



  /*match parse_result {
    Ok((unparsed,tree)) => {
      println!("Unparsed Text: {:?}", unparsed);
      println!("Parse Tree:\n {:?}", tree);
      // let eval_result = eval(tree);
    }
    Err(error) => {
      println!("ERROR {:?}", error);
    }
  }*/
    
  Ok(())
}
