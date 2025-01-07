extern crate chobitlibs;
extern crate alloc;

use chobitlibs::chobit_machine::*;

#[derive(Debug, Clone, PartialEq)]
enum Inst {
    Push(Val),
    Lambda,
    Call,
    Define,
    Get
}

#[derive(Debug, Clone, PartialEq)]
enum Val {
    Int(i32),
    Key(u64),
    Proc(Vec<Inst>),
    Closure(Vec<(u64, Val)>, Vec<Inst>)
}

fn eval(
    stack: &mut ChobitStack<Val>,
    code: &mut ChobitCode<Inst>,
    env: &mut ChobitEnv<Val>
) {
    match code.next() {
        Some(inst) => match inst {
            // Push value.
            Inst::Push(val) => {
                stack.push(val.clone());
            },

            // Lambda.
            Inst::Lambda => {
                let proc = match stack.pop().expect("No values.") {
                    Val::Proc(proc) => proc.clone(),

                    _ => {panic!("Wrong type.")}
                };

                // save environment.
                let mut env_dump = Vec::<(u64, Val)>::new();
                env.dump(&mut env_dump);

                // store saved environment onto closure.
                stack.push(Val::Closure(env_dump, proc));

            },

            // Call
            Inst::Call => {
                let (env_dump, proc) = match stack.pop().expect("No values.") {
                    Val::Closure(env_dump, proc) => (env_dump, proc),

                    _ => {panic!("Wrong type.")}
                };

                code.push_frame();
                code.set_code(&proc);  // load proc of closure.

                env.push_frame();
                env.store(&env_dump);  // load environment of closure
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
        },

        None => {
            code.pop_frame();  // purge proc of closure.
            env.pop_frame();  // purge environment of closure.
        }
    }
}

fn main() {
    let mut stack = ChobitStack::<Val>::new();
    let mut code = ChobitCode::<Inst>::new();
    let mut env = ChobitEnv::<Val>::new();

    // Closure: {key: 100, value: 30} => {key: 100, value: 20}
    let proc: Vec<Inst> = vec![
        Inst::Push(Val::Key(100)),
        Inst::Get,
        Inst::Push(Val::Int(20)),
        Inst::Push(Val::Key(100)),
        Inst::Define,  // {key: 100, value: 20}
        Inst::Push(Val::Key(100)),
        Inst::Get
    ];

    // Define: {key: 100, value: 30}
    code.set_code(&[
        Inst::Push(Val::Int(30)),
        Inst::Push(Val::Key(100)),
        Inst::Define,  // {key: 100, value: 30}
        Inst::Push(Val::Key(100)),
        Inst::Get
    ]);

    eval(&mut stack, &mut code, &mut env);  // Push(Val(30))
    eval(&mut stack, &mut code, &mut env);  // Push(Key(100))
    eval(&mut stack, &mut code, &mut env);  // Define
    assert_eq!(stack.top(), None);

    eval(&mut stack, &mut code, &mut env);  // Push(Key(100))
    eval(&mut stack, &mut code, &mut env);  // Get
    assert_eq!(stack.top(), Some(&Val::Int(30)));

    // Define closure. Env: {key: 100, value: 30}
    code.set_code(&[
        Inst::Push(Val::Proc(proc.clone())),
        Inst::Lambda,  // Save environment. {key: 100, value: 30}
        Inst::Push(Val::Key(200)),
        Inst::Define,
    ]);

    eval(&mut stack, &mut code, &mut env);  // Push(Proc)
    eval(&mut stack, &mut code, &mut env);  // Lambda
    eval(&mut stack, &mut code, &mut env);  // Push(Key(200))
    eval(&mut stack, &mut code, &mut env);  // Define

    // Define: {key: 100, value: 30} => {key: 100, value: 40}
    code.set_code(&[
        Inst::Push(Val::Int(40)),
        Inst::Push(Val::Key(100)),
        Inst::Define,  // {key: 100, value: 40}
        Inst::Push(Val::Key(100)),
        Inst::Get
    ]);

    eval(&mut stack, &mut code, &mut env);  // Push(Val(40))
    eval(&mut stack, &mut code, &mut env);  // Push(Key(100))
    eval(&mut stack, &mut code, &mut env);  // Define
    eval(&mut stack, &mut code, &mut env);  // Push(Key(100))
    eval(&mut stack, &mut code, &mut env);  // Get
    assert_eq!(stack.top(), Some(&Val::Int(40)));

    // Call closure: {key: 100, value: 30} => {key: 100, value: 20}
    code.set_code(&[
        Inst::Push(Val::Key(200)),
        Inst::Get,
        Inst::Call  // Store environment. {key: 100, value: 30}
    ]);

    eval(&mut stack, &mut code, &mut env);  // Push(Key(200))
    eval(&mut stack, &mut code, &mut env);  // Get
    eval(&mut stack, &mut code, &mut env);  // Call

    eval(&mut stack, &mut code, &mut env);  // Push(Key(100))
    eval(&mut stack, &mut code, &mut env);  // Get
    assert_eq!(stack.top(), Some(&Val::Int(30)));

    eval(&mut stack, &mut code, &mut env);  // Push(Val(20))
    eval(&mut stack, &mut code, &mut env);  // Push(Key(100))
    eval(&mut stack, &mut code, &mut env);  // Define
    eval(&mut stack, &mut code, &mut env);  // Push(Key(100))
    eval(&mut stack, &mut code, &mut env);  // Get
    assert_eq!(stack.top(), Some(&Val::Int(20)));

    // pop_frame() => {key: 100, value: 40}
    eval(&mut stack, &mut code, &mut env);

    // After removed closure: {key: 100, value: 40}
    code.set_code(&[
        Inst::Push(Val::Key(100)),
        Inst::Get
    ]);

    eval(&mut stack, &mut code, &mut env);  // Push(Key(100))
    eval(&mut stack, &mut code, &mut env);  // Get
    assert_eq!(stack.top(), Some(&Val::Int(40)));
}
