extern crate chobitlibs;
extern crate alloc;

use chobitlibs::chobit_machine::*;

#[derive(Debug, Clone, PartialEq)]
enum Inst {
    PushInt(i32),
    PushKey(u64),
    Define,
    Get,
    Set
}

#[derive(Debug, Clone, PartialEq)]
enum Val {
    Int(i32),
    Key(u64)
}

fn eval(
    stack: &mut ChobitStack<Val>,
    code: &mut ChobitCode<Inst>,
    env: &mut ChobitEnv<Val>
) {
    match code.next() {
        Some(inst) => match *inst {
            // Push value.
            Inst::PushInt(val) => {
                stack.push(Val::Int(val));
            },

            // Push key.
            Inst::PushKey(key) => {
                stack.push(Val::Key(key));
            },

            // Define.
            Inst::Define => {
                let key = match stack.pop().expect("No values.") {
                    Val::Key(key) => key,

                    _ => {panic!("Wrong type.")}
                };

                let val = stack.pop().expect("No values.");

                env.define(key, val);
            },

            // Get.
            Inst::Get => {
                let key = match stack.pop().expect("No values.") {
                    Val::Key(key) => key,

                    _ => {panic!("Wrong type.")}
                };

                let val = env.get(key).expect("No value in Env.").clone();

                stack.push(val);
            },

            // Set.
            Inst::Set => {
                let key = match stack.pop().expect("No values.") {
                    Val::Key(key) => key,

                    _ => {panic!("Wrong type.")}
                };

                let val = stack.pop().expect("No values.");

                env.set(key, val).expect("No key in Env.");
            },
        },

        None => {println!("Code is over.");}
    }
}

fn main() {
    let mut stack = ChobitStack::<Val>::new();
    let mut code = ChobitCode::<Inst>::new();
    let mut env = ChobitEnv::<Val>::new();

    // Define: {key: 100, val: 2}
    code.set_code(&[
        Inst::PushInt(2),
        Inst::PushKey(100),
        Inst::Define
    ]);

    eval(&mut stack, &mut code, &mut env);  // PushInt(2)
    eval(&mut stack, &mut code, &mut env);  // PushKey(100)
    eval(&mut stack, &mut code, &mut env);  // Define

    assert_eq!(stack.pop(), None);

    // Get: {key: 100} => 2
    code.set_code(&[
        Inst::PushKey(100),
        Inst::Get
    ]);

    eval(&mut stack, &mut code, &mut env);  // PushKey(100)
    eval(&mut stack, &mut code, &mut env);  // Get

    assert_eq!(stack.pop(), Some(Val::Int(2)));

    // Set: {key: 100, val: 3}
    code.set_code(&[
        Inst::PushInt(3),
        Inst::PushKey(100),
        Inst::Set
    ]);

    eval(&mut stack, &mut code, &mut env);  // PushInt(3)
    eval(&mut stack, &mut code, &mut env);  // PushKey(100)
    eval(&mut stack, &mut code, &mut env);  // Set

    assert_eq!(stack.pop(), None);

    // Get: {key: 100} => 3
    code.set_code(&[
        Inst::PushKey(100),
        Inst::Get
    ]);

    eval(&mut stack, &mut code, &mut env);  // PushKey(100)
    eval(&mut stack, &mut code, &mut env);  // Get

    assert_eq!(stack.pop(), Some(Val::Int(3)));
}
