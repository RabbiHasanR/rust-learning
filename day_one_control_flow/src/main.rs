mod block_scope;
mod if_expression;
mod match_expression;
mod loops;
mod break_continue;
mod labels;
mod functions;


fn main() {
    println!("This is a simple Rust program demonstrating block scope.");
    block_scope::block_scope();
    if_expression::if_expression(); 
    let result = if_expression::if_expression_with_return();
    println!("Result from if_expression_with_return: {}", result);
    if_expression::if_expression_with_one_line();
    match_expression::match_expression();
    match_expression::match_expression_with_return();

    loops::while_loop();
    loops::for_loop();
    loops::loop_statement();

    break_continue::break_continue();

    let return_value = break_continue::break_return();
    println!("Return value from break_return: {}", return_value);

    labels::labels_break();

    functions::functions();
    println!("Functions executed successfully.");
    println!("End of the program.");
}
