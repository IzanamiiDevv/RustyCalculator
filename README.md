# RustyCalculator

This is a simple Reverse Polish Notation (RPN) calculator implemented in Rust. RPN is a mathematical notation in which every operator follows all of its operands, and there are no parentheses to indicate precedence. This calculator evaluates arithmetic expressions written in postfix notation.

## Usage

### Installation

Ensure you have Rust installed on your system. If not, you can install it from [here](https://www.rust-lang.org/tools/install).

### Running the Calculator

To use the calculator, run the following command:

```bash
cargo run -- -text "<your_expression>"
```

Replace `<your_expression>` with the infix arithmetic expression you want to evaluate. For example:

```bash
cargo run -- -text "3 * (4 + 2) - 7 / 2"
```

### Output

The program will tokenize the input expression, convert it to RPN (Reverse Polish Notation), and then evaluate the RPN expression, printing out the result.

## Implementation Details

### Tokenization

The input expression is tokenized into individual components such as digits, operators, left and right parentheses. This is achieved by iterating through each character of the input string and classifying them accordingly.

### Infix to RPN Conversion

The Shunting Yard algorithm is used to convert the infix expression to RPN. This algorithm utilizes two stacks: one for operators and another for the output (RPN expression). It scans the input expression from left to right, pushing operands directly to the output and handling operators and parentheses according to their precedence and associativity.

### Evaluation

Once the expression is in RPN form, it is evaluated using a stack-based approach. Each token (operand or operator) is processed one by one. Operands are pushed onto the stack, and when an operator is encountered, the necessary number of operands are popped from the stack, the operation is performed, and the result is pushed back onto the stack.

## Error Handling

The program performs basic error handling for invalid expressions, including division by zero, insufficient operands for operators, and invalid tokens.

## Examples

Example usage of the calculator:

```bash
cargo run -- -text "3 * (4 + 2) - 7 / 2"
```

Output:
```
Expression: 3 * (4 + 2) - 7 / 2

Tokens: [
    Digit("3"),
    Operator("*"),
    LeftParen,
    Digit("4"),
    Operator("+"),
    Digit("2"),
    RightParen,
    Operator("-"),
    Digit("7"),
    Operator("/"),
    Digit("2")
]

RPN: [
    Digit("3"),
    Digit("4"),
    Digit("2"), 
    Operator("+"), 
    Operator("*"), 
    Digit("7"), 
    Digit("2"), 
    Operator("/"), 
    Operator("-")
]

Result: 14
```