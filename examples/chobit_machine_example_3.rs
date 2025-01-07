extern crate chobitlibs;
extern crate alloc;

use chobitlibs::chobit_machine::*;

#[derive(Debug, Clone, PartialEq)]
enum Inst {
    PushInt(i32),
    PushBool(bool),
    PushProc(Vec<Inst>),
    IfElse
}

#[derive(Debug, Clone, PartialEq)]
enum Val {
    Int(i32),
    Bool(bool),
    Proc(Vec<Inst>)
}

fn eval(
    stack: &mut ChobitStack<Val>,
    code: &mut ChobitCode<Inst>
) {
    match code.next() {
        Some(inst) => match inst {
            // Push value.
            Inst::PushInt(val) => {
                stack.push(Val::Int(*val));
            },

            // Push boolean value.
            Inst::PushBool(val) => {
                stack.push(Val::Bool(*val));
            },

            // Push code.
            Inst::PushProc(val) => {
                stack.push(Val::Proc(val.clone()));
            },

            // IfElse.
            Inst::IfElse => {
                let else_ = match stack.pop().expect("No values.") {
                    Val::Proc(code) => code,

                    _ => {panic!("Wrong type.")}
                };

                let if_ = match stack.pop().expect("No values.") {
                    Val::Proc(code) => code,

                    _ => {panic!("Wrong type.")}
                };

                let cond = match stack.pop().expect("No values.") {
                    Val::Bool(val) => val,

                    _ => {panic!("Wrong type.")}
                };

                code.push_frame();
                if cond {
                    code.set_code(&if_);
                } else {
                    code.set_code(&else_);
                }
            },
        },

        None => {panic!("Code is over.");}
    }
}

fn main() {
    let mut stack = ChobitStack::<Val>::new();
    let mut code = ChobitCode::<Inst>::new();

    let if_: Vec<Inst> = vec![Inst::PushInt(2)];  // if true => 2
    let else_: Vec<Inst> = vec![Inst::PushInt(3)];  // if false => 3

    // if true then 2 else 3 => 2
    code.set_code(&[
        Inst::PushBool(true),
        Inst::PushProc(if_.clone()),
        Inst::PushProc(else_.clone()),
        Inst::IfElse
    ]);

    eval(&mut stack, &mut code);  // PushBool(true)
    eval(&mut stack, &mut code);  // PushProc(if_)
    eval(&mut stack, &mut code);  // PushProc(else_)
    eval(&mut stack, &mut code);  // IfElse
    eval(&mut stack, &mut code);  // PushInt(2)

    assert_eq!(stack.top(), Some(&Val::Int(2)));

    code.pop_frame();

    // if false then 2 else 3 => 3
    code.set_code(&[
        Inst::PushBool(false),
        Inst::PushProc(if_.clone()),
        Inst::PushProc(else_.clone()),
        Inst::IfElse
    ]);

    eval(&mut stack, &mut code);  // PushBool(false)
    eval(&mut stack, &mut code);  // PushProc(if_)
    eval(&mut stack, &mut code);  // PushProc(else_)
    eval(&mut stack, &mut code);  // IfElse
    eval(&mut stack, &mut code);  // PushInt(3)

    assert_eq!(stack.pop(), Some(Val::Int(3)));

    code.pop_frame();
}
