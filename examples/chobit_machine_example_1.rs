extern crate chobitlibs;
extern crate alloc;

use chobitlibs::chobit_machine::*;

#[derive(Debug, Clone, PartialEq)]
enum Inst {
    Push(i32),
    Add,
    Mul
}

#[derive(Debug, Clone, PartialEq)]
enum Val {
    Int(i32)
}

fn eval(
    stack: &mut ChobitStack<Val>,
    code: &mut ChobitCode<Inst>
) {
    match code.next() {
        Some(inst) => match *inst {
            // Push value.
            Inst::Push(val) => {
                stack.push(Val::Int(val));
            },

            // Addition.
            // Pop val_1.
            // Pop val_2.
            // Push (val_1 + val_2).
            Inst::Add => {
                let val_1 = match stack.pop().expect("No values.") {
                    Val::Int(val) => val
                };

                let val_2 = match stack.pop().expect("No values.") {
                    Val::Int(val) => val
                };

                stack.push(Val::Int(val_1 + val_2));
            }

            // Multiplication
            // Pop val_1.
            // Pop val_2.
            // Push (val_1 * val_2).
            Inst::Mul => {
                let val_1 = match stack.pop().expect("No values.") {
                    Val::Int(val) => val
                };

                let val_2 = match stack.pop().expect("No values.") {
                    Val::Int(val) => val
                };

                stack.push(Val::Int(val_1 * val_2));
            }
        },

        None => {println!("Code is over.");}
    }
}

fn main() {
    let mut stack = ChobitStack::<Val>::new();
    let mut code = ChobitCode::<Inst>::new();

    // 2 + 3
    code.set_code(&[
        Inst::Push(2),
        Inst::Push(3),
        Inst::Add
    ]);

    eval(&mut stack, &mut code);  // Push(2)
    eval(&mut stack, &mut code);  // Push(3)
    eval(&mut stack, &mut code);  // Add

    assert_eq!(stack.pop(), Some(Val::Int(2 + 3)));

    // 2 * 3
    code.set_code(&[
        Inst::Push(2),
        Inst::Push(3),
        Inst::Mul
    ]);

    eval(&mut stack, &mut code);  // Push(2)
    eval(&mut stack, &mut code);  // Push(3)
    eval(&mut stack, &mut code);  // Mul

    assert_eq!(stack.pop(), Some(Val::Int(2 * 3)));

    // (2 + 3) * (4 + 5)
    code.set_code(&[
        Inst::Push(2),
        Inst::Push(3),
        Inst::Add,
        Inst::Push(4),
        Inst::Push(5),
        Inst::Add,
        Inst::Mul
    ]);

    eval(&mut stack, &mut code);  // Push(2)
    eval(&mut stack, &mut code);  // Push(3)
    eval(&mut stack, &mut code);  // Add
    assert_eq!(stack.top(), Some(&Val::Int(2 + 3)));

    eval(&mut stack, &mut code);  // Push(4)
    eval(&mut stack, &mut code);  // Push(5)
    eval(&mut stack, &mut code);  // Add
    assert_eq!(stack.top(), Some(&Val::Int(4 + 5)));

    eval(&mut stack, &mut code);  // Mul

    assert_eq!(stack.pop(), Some(Val::Int((2 + 3) * (4 + 5))));
}
