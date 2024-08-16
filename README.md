# SimpleParser_Rust

Extended the parser implementation in /src/parser.rs to support the grammar defined in grammar.ebnf. Initially, the program() function only handled easy cases such as numbers and identifiers. The updated implementation now parses more complex language constructs, as specified by the grammar, enabling better parsing of the language. This meant modifying program() and ensuring that it can handle different syntactic structures beyond basic elements, aligning the parser with the Rust language specification.

This does contain a main.rs file; Still, project was done with test cases passing in mind. Therefore, we do noy pay much attention to "cargo run" for our main.rs output. The main.rs only has one test case.


## Required Features

The following functionality is implemented:

- [X] Enhanced program() function to handle complex language constructs according to the grammar.ebnf specification.
- [X] Developed additional parser combinators to manage various grammar nodes such as function calls, variable definitions, and boolean expressions.
- [X] Integrated nom library combinators to effectively combine and parse grammar rules.
- [X] Addressed and resolved issues in parsing tests to ensure accurate handling of function calls, variable definitions, and other language features.

## How to run

1. Run "cargo build" to compile the project
2. Run "cargo test" to check the passing test cases
3. Run "cargo run" to actually run the project. In this case, this is not very important as main.rs only includes one test case. Therefore, running "cargo test" is enough to verify the interpreter is working with our test case inputs.
4. Enjoy the project!

## Video Walkthrough

N/A

## Notes

N/A

## License

Copyright 2024 Samir Hassan

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

> http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
