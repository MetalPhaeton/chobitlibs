extern crate chobitlibs;
extern crate turbo_json_checker;

use chobitlibs::chobit_machine::*;
use turbo_json_checker as tjc;

#[test]
fn test_stack_error() {
    assert!(tjc::validate_str(
        &(ChobitStackError::WrongBp {bp: 10}).to_string()
    ).is_ok());

    assert!(tjc::validate_str(
        &(ChobitStackError::WrongFrameStack {index: 20, bp: 30}).to_string()
    ).is_ok());
}

#[test]
fn test_stack_load() {
    let body: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let bp: usize = 7;
    let frame_stack: Vec<usize> = vec![0, 2, 4];

    let stack = ChobitStack::load(
        &body,
        bp,
        &frame_stack
    ).unwrap();

    assert_eq!(stack.body(), body.as_slice());
    assert_eq!(stack.bp(), bp);
    assert_eq!(stack.frame_stack(), frame_stack.as_slice());

    let (body_2, bp_2, frame_stack_2) = stack.drop();

    assert_eq!(body_2.as_slice(), body.as_slice());
    assert_eq!(bp_2, bp);
    assert_eq!(frame_stack_2.as_slice(), frame_stack.as_slice());
}

#[test]
fn test_stack_load_error() {
    let body: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let bp: usize = 10;
    let frame_stack: Vec<usize> = vec![0, 2, 4];

    assert!(ChobitStack::load(
        &body,
        bp,
        &frame_stack
    ).is_ok());

    let bp: usize = 11;
    assert_eq!(
        ChobitStack::load(
            &body,
            bp,
            &frame_stack
        ).err().unwrap(),
        ChobitStackError::WrongBp {bp: bp}
    );

    let bp: usize = 5;
    assert!(ChobitStack::load(
        &body,
        bp,
        &frame_stack
    ).is_ok());

    let bp: usize = 4;
    assert!(ChobitStack::load(
        &body,
        bp,
        &frame_stack
    ).is_ok());

    let bp: usize = 3;
    assert_eq!(
        ChobitStack::load(
            &body,
            bp,
            &frame_stack
        ).err().unwrap(),
        ChobitStackError::WrongFrameStack {index: 2, bp: frame_stack[2]}
    );

    let bp: usize = 9;
    let frame_stack: Vec<usize> = vec![0, 2, 2];
    assert!(ChobitStack::load(
        &body,
        bp,
        &frame_stack
    ).is_ok());

    let frame_stack: Vec<usize> = vec![0, 3, 2];
    assert_eq!(
        ChobitStack::load(
            &body,
            bp,
            &frame_stack
        ).err().unwrap(),
        ChobitStackError::WrongFrameStack {index: 1, bp: frame_stack[1]}
    );

    let frame_stack: Vec<usize> = vec![2, 1, 2];
    assert_eq!(
        ChobitStack::load(
            &body,
            bp,
            &frame_stack
        ).err().unwrap(),
        ChobitStackError::WrongFrameStack {index: 0, bp: frame_stack[0]}
    );
}

#[test]
fn test_stack_push_pop_top() {
    let body: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let bp: usize = 7;
    let frame_stack: Vec<usize> = vec![0, 2, 4];

    let mut stack = ChobitStack::load(
        &body,
        bp,
        &frame_stack
    ).unwrap();

    let push_value: i32 = 20;

    stack.push(push_value);

    assert_eq!(stack.body().len(), body.len() + 1);
    assert_eq!(*stack.top().unwrap(), push_value);

    let push_value_2 = stack.pop().unwrap();
    assert_eq!(push_value_2, push_value);
}

#[test]
fn test_stack_push_pop_frame() {
    let body_1: Vec<i32> = vec![1, 2, 3];
    let body_2: Vec<i32> = vec![4, 5];
    let body_3: Vec<i32> = vec![6, 7];
    let body_4: Vec<i32> = vec![8, 9, 10];
    let bp_1: usize = 0;
    let bp_2: usize = bp_1 + body_1.len();
    let bp_3: usize = bp_2 + body_2.len();
    let bp_4: usize = bp_3 + body_3.len();

    let mut body = Vec::<i32>::new();
    let mut frame_stack = Vec::<usize>::new();
    let mut stack = ChobitStack::<i32>::new();

    // push ----------
    assert_eq!(stack.body(), &body);
    assert_eq!(stack.current_frame(), &[]);
    assert_eq!(stack.bp(), bp_1);
    assert_eq!(stack.frame_stack(), &frame_stack);

    body.extend_from_slice(&body_1);
    body_1.iter().for_each(|val| {stack.push(*val);});

    assert_eq!(stack.body(), &body);
    assert_eq!(stack.current_frame(), &body_1);
    assert_eq!(stack.bp(), bp_1);
    assert_eq!(stack.frame_stack(), &frame_stack);

    stack.push_frame();
    frame_stack.push(bp_1);
    assert_eq!(stack.body(), &body);
    assert_eq!(stack.current_frame(), &[]);
    assert_eq!(stack.bp(), bp_2);
    assert_eq!(stack.frame_stack(), &frame_stack);

    body.extend_from_slice(&body_2);
    body_2.iter().for_each(|val| {stack.push(*val);});

    assert_eq!(stack.body(), &body);
    assert_eq!(stack.current_frame(), &body_2);
    assert_eq!(stack.bp(), bp_2);
    assert_eq!(stack.frame_stack(), &frame_stack);

    stack.push_frame();
    frame_stack.push(bp_2);
    assert_eq!(stack.body(), &body);
    assert_eq!(stack.current_frame(), &[]);
    assert_eq!(stack.bp(), bp_3);
    assert_eq!(stack.frame_stack(), &frame_stack);

    body.extend_from_slice(&body_3);
    body_3.iter().for_each(|val| {stack.push(*val);});

    assert_eq!(stack.body(), &body);
    assert_eq!(stack.current_frame(), &body_3);
    assert_eq!(stack.bp(), bp_3);
    assert_eq!(stack.frame_stack(), &frame_stack);

    stack.push_frame();
    frame_stack.push(bp_3);
    assert_eq!(stack.body(), &body);
    assert_eq!(stack.current_frame(), &[]);
    assert_eq!(stack.bp(), bp_4);
    assert_eq!(stack.frame_stack(), &frame_stack);

    body.extend_from_slice(&body_4);
    body_4.iter().for_each(|val| {stack.push(*val);});

    assert_eq!(stack.body(), &body);
    assert_eq!(stack.current_frame(), &body_4);
    assert_eq!(stack.bp(), bp_4);
    assert_eq!(stack.frame_stack(), &frame_stack);

    // pop ----------
    assert!(stack.pop_frame());
    body.truncate(bp_4);
    let _ = frame_stack.pop().unwrap();

    assert_eq!(stack.body(), &body);
    assert_eq!(stack.current_frame(), &body_3);
    assert_eq!(stack.bp(), bp_3);
    assert_eq!(stack.frame_stack(), &frame_stack);

    assert!(stack.pop_frame());
    body.truncate(bp_3);
    let _ = frame_stack.pop().unwrap();

    assert_eq!(stack.body(), &body);
    assert_eq!(stack.current_frame(), &body_2);
    assert_eq!(stack.bp(), bp_2);
    assert_eq!(stack.frame_stack(), &frame_stack);

    assert!(stack.pop_frame());
    body.truncate(bp_2);
    frame_stack.clear();

    assert_eq!(stack.body(), &body);
    assert_eq!(stack.current_frame(), &body_1);
    assert_eq!(stack.bp(), bp_1);
    assert_eq!(stack.frame_stack(), &frame_stack);

    assert!(stack.pop_frame());
    body.clear();

    assert_eq!(stack.body(), &body);
    assert_eq!(stack.current_frame(), &[]);
    assert_eq!(stack.bp(), bp_1);
    assert_eq!(stack.frame_stack(), &frame_stack);
}

#[test]
fn test_code_error() {
    assert!(tjc::validate_str(
        &(ChobitCodeError::WrongBp {bp: 10}).to_string()
    ).is_ok());

    assert!(tjc::validate_str(
        &(ChobitCodeError::WrongIp {ip: 10}).to_string()
    ).is_ok());

    assert!(tjc::validate_str(&(ChobitCodeError::WrongFrameStack {
        index: 20,
        bp: 30,
        bp_ip: 40
    }).to_string()).is_ok());
}

#[test]
fn test_code_load() {
    let body: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let bp: usize = 7;
    let ip: usize = 1;
    let frame_stack: Vec<(usize, usize)> = vec![(0, 1), (2, 2), (3, 5)];

    let code = ChobitCode::load(
        &body,
        bp,
        ip,
        &frame_stack
    ).unwrap();

    assert_eq!(code.ip(), ip);
    assert_eq!(code.bp_ip(), ip + bp);

    let (body_2, bp_2, ip_2, frame_stack_2) = code.drop();

    assert_eq!(body_2, body);
    assert_eq!(bp_2, bp);
    assert_eq!(ip_2, ip);
    assert_eq!(frame_stack_2, frame_stack);
}

#[test]
fn test_code_load_error() {
    let body: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let bp: usize = 10;
    let ip: usize = 0;
    let frame_stack: Vec<(usize, usize)> = vec![(0, 1), (2, 2), (3, 5)];

    assert!(ChobitCode::load(
        &body,
        bp,
        ip,
        &frame_stack
    ).is_ok());

    let bp: usize = 11;
    let ip: usize = 0;
    assert_eq!(
        ChobitCode::load(
            &body,
            bp,
            ip,
            &frame_stack
        ).err().unwrap(),
        ChobitCodeError::WrongBp {bp: bp}
    );

    let bp: usize = 7;
    let ip: usize = 4;
    assert_eq!(
        ChobitCode::load(
            &body,
            bp,
            ip,
            &frame_stack
        ).err().unwrap(),
        ChobitCodeError::WrongIp {ip: ip}
    );

    let bp: usize = 7;
    let ip: usize = 2;
    let frame_stack: Vec<(usize, usize)> = vec![(0, 1), (2, 1), (3, 5)];
    assert_eq!(
        ChobitCode::load(
            &body,
            bp,
            ip,
            &frame_stack
        ).err().unwrap(),
        ChobitCodeError::WrongFrameStack {
            index: 1,
            bp: 2,
            bp_ip: 1
        }
    );

    let bp: usize = 7;
    let ip: usize = 2;
    let frame_stack: Vec<(usize, usize)> = vec![(0, 1), (2, 3), (3, 5)];
    assert!(ChobitCode::load(
        &body,
        bp,
        ip,
        &frame_stack
    ).is_ok());

    let bp: usize = 7;
    let ip: usize = 2;
    let frame_stack: Vec<(usize, usize)> = vec![(0, 1), (2, 4), (3, 5)];
    assert_eq!(
        ChobitCode::load(
            &body,
            bp,
            ip,
            &frame_stack
        ).err().unwrap(),
        ChobitCodeError::WrongFrameStack {
            index: 1,
            bp: 2,
            bp_ip: 4
        }
    );

    let bp: usize = 7;
    let ip: usize = 2;
    let frame_stack: Vec<(usize, usize)> = vec![(0, 1), (3, 3), (3, 5)];
    assert!(ChobitCode::load(
        &body,
        bp,
        ip,
        &frame_stack
    ).is_ok());

    let bp: usize = 7;
    let ip: usize = 2;
    let frame_stack: Vec<(usize, usize)> = vec![(0, 1), (4, 4), (3, 5)];
    assert_eq!(
        ChobitCode::load(
            &body,
            bp,
            ip,
            &frame_stack
        ).err().unwrap(),
        ChobitCodeError::WrongFrameStack {
            index: 1,
            bp: 4,
            bp_ip: 4
        }
    );

    let bp: usize = 5;
    let ip: usize = 2;
    let frame_stack: Vec<(usize, usize)> = vec![(0, 1), (2, 2), (3, 5)];
    assert!(ChobitCode::load(
        &body,
        bp,
        ip,
        &frame_stack
    ).is_ok());

    let bp: usize = 4;
    let ip: usize = 2;
    let frame_stack: Vec<(usize, usize)> = vec![(0, 1), (2, 2), (3, 5)];
    assert_eq!(
        ChobitCode::load(
            &body,
            bp,
            ip,
            &frame_stack
        ).err().unwrap(),
        ChobitCodeError::WrongFrameStack {
            index: 2,
            bp: 3,
            bp_ip: 5
        }
    );
}

#[test]
fn test_code_others() {
    let body_1: Vec<i32> = vec![1, 2, 3];
    let body_2: Vec<i32> = vec![4, 5];
    let body_3: Vec<i32> = vec![6, 7];
    let bp_1: usize = 0;
    let bp_2: usize = bp_1 + body_1.len();
    let bp_3: usize = bp_2 + body_2.len();

    let mut code = ChobitCode::<i32>::new();

    let mut body = Vec::<i32>::new();
    let mut frame_stack = Vec::<(usize, usize)>::new();

    // push_frame --------

    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &[]);
    assert_eq!(code.frame_stack(), &frame_stack);
    assert_eq!(code.bp(), bp_1);
    assert_eq!(code.ip(), 0);
    assert_eq!(code.bp_ip(), bp_1 + 0);

    body.extend_from_slice(&body_1);

    code.set_code(&body_1);

    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &body_1);
    assert_eq!(code.bp(), bp_1);
    assert_eq!(code.ip(), 0);
    assert_eq!(code.bp_ip(), bp_1 + 0);

    let ip = code.ip();
    assert_eq!(code.bp(), bp_1);
    assert_eq!(ip, 0);
    assert_eq!(code.bp_ip(), bp_1 + ip);

    assert_eq!(*code.next().unwrap(), body_1[ip]);
    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &body_1);
    assert_eq!(code.frame_stack(), &frame_stack);

    let ip = code.ip();
    assert_eq!(code.bp(), bp_1);
    assert_eq!(ip, 1);
    assert_eq!(code.bp_ip(), bp_1 + ip);

    assert_eq!(*code.next().unwrap(), body_1[ip]);
    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &body_1);
    assert_eq!(code.frame_stack(), &frame_stack);

    let ip = code.ip();
    assert_eq!(code.bp(), bp_1);
    assert_eq!(ip, 2);
    assert_eq!(code.bp_ip(), bp_1 + ip);

    assert_eq!(*code.next().unwrap(), body_1[ip]);
    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &body_1);
    assert_eq!(code.frame_stack(), &frame_stack);

    let ip = code.ip();
    assert_eq!(code.bp(), bp_1);
    assert_eq!(ip, 3);
    assert_eq!(code.bp_ip(), bp_1 + ip);

    assert_eq!(code.next(), None);
    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &body_1);
    assert_eq!(code.frame_stack(), &frame_stack);

    code.rewind();

    let ip = code.ip();
    assert_eq!(code.bp(), bp_1);
    assert_eq!(ip, 0);
    assert_eq!(code.bp_ip(), bp_1 + ip);

    assert_eq!(*code.next().unwrap(), body_1[ip]);
    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &body_1);
    assert_eq!(code.frame_stack(), &frame_stack);

    assert!(code.set_ip(2).is_ok());

    let ip = code.ip();
    assert_eq!(code.bp(), bp_1);
    assert_eq!(ip, 2);
    assert_eq!(code.bp_ip(), bp_1 + ip);

    assert_eq!(*code.next().unwrap(), body_1[ip]);
    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &body_1);
    assert_eq!(code.frame_stack(), &frame_stack);

    assert_eq!(
        code.set_ip(4),
        Err(ChobitCodeError::WrongIp {ip: 4})
    );

    let ip = code.ip();
    assert_eq!(code.bp(), bp_1);
    assert_eq!(ip, 3);
    assert_eq!(code.bp_ip(), bp_1 + ip);

    let ip_1 = code.ip();

    code.push_frame();
    frame_stack.push((bp_1, bp_1 + ip_1));

    let ip = code.ip();
    assert_eq!(code.bp(), bp_2);
    assert_eq!(ip, 0);
    assert_eq!(code.bp_ip(), bp_2 + ip);

    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &[]);
    assert_eq!(code.frame_stack(), &frame_stack);

    body.extend_from_slice(&body_2);
    code.set_code(&body_2);
    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &body_2);
    assert_eq!(code.frame_stack(), &frame_stack);

    assert_eq!(*code.next().unwrap(), body_2[ip]);
    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &body_2);
    assert_eq!(code.frame_stack(), &frame_stack);

    let ip = code.ip();
    assert_eq!(code.bp(), bp_2);
    assert_eq!(ip, 1);
    assert_eq!(code.bp_ip(), bp_2 + ip);

    assert_eq!(*code.next().unwrap(), body_2[ip]);
    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &body_2);
    assert_eq!(code.frame_stack(), &frame_stack);

    let ip = code.ip();
    assert_eq!(code.bp(), bp_2);
    assert_eq!(ip, 2);
    assert_eq!(code.bp_ip(), bp_2 + ip);

    assert_eq!(code.next(), None);

    let ip = code.ip();
    assert_eq!(code.bp(), bp_2);
    assert_eq!(ip, 2);
    assert_eq!(code.bp_ip(), bp_2 + ip);

    code.rewind();

    let ip = code.ip();
    assert_eq!(code.bp(), bp_2);
    assert_eq!(ip, 0);
    assert_eq!(code.bp_ip(), bp_2 + ip);

    assert_eq!(
        code.set_ip(3),
        Err(ChobitCodeError::WrongIp {ip: 3})
    );
    assert!(code.set_ip(2).is_ok());

    let ip_2 = code.ip();
    assert_eq!(code.bp(), bp_2);
    assert_eq!(ip_2, 2);
    assert_eq!(code.bp_ip(), bp_2 + ip_2);
    assert_eq!(code.frame_stack(), &frame_stack);

    code.push_frame();
    frame_stack.push((bp_2, bp_2 + ip_2));

    let ip = code.ip();
    assert_eq!(code.bp(), bp_3);
    assert_eq!(ip, 0);
    assert_eq!(code.bp_ip(), bp_3 + ip);

    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &[]);
    assert_eq!(code.frame_stack(), &frame_stack);

    body.extend_from_slice(&body_3);
    code.set_code(&body_3);
    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &body_3);
    assert_eq!(code.frame_stack(), &frame_stack);

    let ip = code.ip();
    assert_eq!(code.bp(), bp_3);
    assert_eq!(ip, 0);
    assert_eq!(code.bp_ip(), bp_3 + ip);

    assert_eq!(*code.next().unwrap(), body_3[ip]);
    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &body_3);
    assert_eq!(code.frame_stack(), &frame_stack);

    let ip = code.ip();
    assert_eq!(code.bp(), bp_3);
    assert_eq!(ip, 1);
    assert_eq!(code.bp_ip(), bp_3 + ip);

    assert_eq!(*code.next().unwrap(), body_3[ip]);
    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &body_3);
    assert_eq!(code.frame_stack(), &frame_stack);

    let ip = code.ip();
    assert_eq!(code.bp(), bp_3);
    assert_eq!(ip, 2);
    assert_eq!(code.bp_ip(), bp_3 + ip);

    assert_eq!(code.next(), None);

    let ip = code.ip();
    assert_eq!(code.bp(), bp_3);
    assert_eq!(ip, 2);
    assert_eq!(code.bp_ip(), bp_3 + ip);

    code.rewind();

    let ip = code.ip();
    assert_eq!(code.bp(), bp_3);
    assert_eq!(ip, 0);
    assert_eq!(code.bp_ip(), bp_3 + ip);

    assert_eq!(*code.next().unwrap(), body_3[ip]);
    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &body_3);
    assert_eq!(code.frame_stack(), &frame_stack);

    let ip = code.ip();
    assert_eq!(code.bp(), bp_3);
    assert_eq!(ip, 1);
    assert_eq!(code.bp_ip(), bp_3 + ip);

    assert_eq!(
        code.set_ip(3),
        Err(ChobitCodeError::WrongIp {ip: 3})
    );

    let ip = code.ip();
    assert_eq!(code.bp(), bp_3);
    assert_eq!(ip, 1);
    assert_eq!(code.bp_ip(), bp_3 + ip);

    assert!(code.set_ip(2).is_ok());

    let ip = code.ip();
    assert_eq!(code.bp(), bp_3);
    assert_eq!(ip, 2);
    assert_eq!(code.bp_ip(), bp_3 + ip);

    // pop_frame --------

    assert!(code.pop_frame());
    let _ = frame_stack.pop();
    body.truncate(bp_3);

    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &body_2);
    assert_eq!(code.frame_stack(), &frame_stack);
    assert_eq!(code.bp(), bp_2);
    assert_eq!(code.ip(), ip_2);
    assert_eq!(code.bp_ip(), bp_2 + ip_2);

    assert!(code.pop_frame());
    let _ = frame_stack.pop();
    body.truncate(bp_2);

    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &body_1);
    assert_eq!(code.frame_stack(), &frame_stack);
    assert_eq!(code.bp(), bp_1);
    assert_eq!(code.ip(), ip_1);
    assert_eq!(code.bp_ip(), bp_1 + ip_1);

    assert!(code.pop_frame());
    let _ = frame_stack.pop();
    body.clear();

    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &[]);
    assert_eq!(code.frame_stack(), &frame_stack);
    assert_eq!(code.bp(), 0);
    assert_eq!(code.ip(), 0);
    assert_eq!(code.bp_ip(), 0);

    assert!(!code.pop_frame());

    assert_eq!(code.body(), &body);
    assert_eq!(code.current_frame(), &[]);
    assert_eq!(code.frame_stack(), &frame_stack);
    assert_eq!(code.bp(), 0);
    assert_eq!(code.ip(), 0);
    assert_eq!(code.bp_ip(), 0);
}

#[test]
fn test_env_error() {
    assert!(tjc::validate_str(
        &(ChobitEnvError::WrongBp {bp: 10}).to_string()
    ).is_ok());

    assert!(tjc::validate_str(
        &(ChobitEnvError::WrongFrameStack {index: 20, bp: 30}).to_string()
    ).is_ok());

    assert!(tjc::validate_str(
        &(ChobitEnvError::NotFound {key: 40}).to_string()
    ).is_ok());
}

#[test]
fn test_env_load() {
    let keys: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let values: Vec<i32> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let bp: usize = 7;
    let frame_stack: Vec<usize> = vec![0, 2, 4];
    let body: Vec<(u64, i32)> = keys
        .iter()
        .zip(values.iter())
        .map(|(key, value)| (*key, *value))
        .collect();

    let env = ChobitEnv::load(
        &body,
        bp,
        &frame_stack
    ).unwrap();

    assert_eq!(env.keys(), keys.as_slice());
    assert_eq!(env.values(), values.as_slice());
    assert_eq!(env.bp(), bp);
    assert_eq!(env.frame_stack(), frame_stack.as_slice());

    let (body_2, bp_2, frame_stack_2) = env.drop();

    assert_eq!(body_2.as_slice(), body.as_slice());
    assert_eq!(bp_2, bp);
    assert_eq!(frame_stack_2.as_slice(), frame_stack.as_slice());
}

#[test]
fn test_env_load_error() {
    let keys: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let values: Vec<i32> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let bp: usize = 7;
    let frame_stack: Vec<usize> = vec![0, 2, 4];
    let body: Vec<(u64, i32)> = keys
        .iter()
        .zip(values.iter())
        .map(|(key, value)| (*key, *value))
        .collect();

    assert!(ChobitEnv::load(
        &body,
        bp,
        &frame_stack
    ).is_ok());

    let bp: usize = 11;
    assert_eq!(
        ChobitEnv::load(
            &body,
            bp,
            &frame_stack
        ).err().unwrap(),
        ChobitEnvError::WrongBp {bp: bp}
    );

    let bp: usize = 5;
    assert!(ChobitEnv::load(
        &body,
        bp,
        &frame_stack
    ).is_ok());

    let bp: usize = 4;
    assert!(ChobitEnv::load(
        &body,
        bp,
        &frame_stack
    ).is_ok());

    let bp: usize = 3;
    assert_eq!(
        ChobitEnv::load(
            &body,
            bp,
            &frame_stack
        ).err().unwrap(),
        ChobitEnvError::WrongFrameStack {index: 2, bp: frame_stack[2]}
    );

    let bp: usize = 9;
    let frame_stack: Vec<usize> = vec![0, 2, 2];
    assert!(ChobitEnv::load(
        &body,
        bp,
        &frame_stack
    ).is_ok());

    let frame_stack: Vec<usize> = vec![0, 3, 2];
    assert_eq!(
        ChobitEnv::load(
            &body,
            bp,
            &frame_stack
        ).err().unwrap(),
        ChobitEnvError::WrongFrameStack {index: 1, bp: frame_stack[1]}
    );

    let frame_stack: Vec<usize> = vec![2, 1, 2];
    assert_eq!(
        ChobitEnv::load(
            &body,
            bp,
            &frame_stack
        ).err().unwrap(),
        ChobitEnvError::WrongFrameStack {index: 0, bp: frame_stack[0]}
    );
}

#[test]
fn test_env_others() {
    let keys_1: Vec<u64> = vec![1, 2, 1];
    let keys_2: Vec<u64> = vec![4, 5];
    let keys_3: Vec<u64> = vec![6, 4];
    let values_1: Vec<i32> = vec![10, 20, 30];
    let values_2: Vec<i32> = vec![40, 50];
    let values_3: Vec<i32> = vec![60, 70];

    let values_11: Vec<i32> = vec![10, 200, 300];
    let values_12: Vec<i32> = vec![400, 500];
    let values_13: Vec<i32> = vec![600, 700];
    let bp_1: usize = 0;
    let bp_2: usize = bp_1 + keys_1.len();
    let bp_3: usize = bp_2 + keys_2.len();

    let mut env = ChobitEnv::<i32>::new();

    let mut keys = Vec::<u64>::new();
    let mut values = Vec::<i32>::new();
    let mut frame_stack = Vec::<usize>::new();
    let mut keys_values = Vec::<(u64, i32)>::new();
    let mut keys_values_out = Vec::<(u64, i32)>::new();

    // push_frame --------

    assert_eq!(env.keys(), &keys);
    assert_eq!(env.values(), &values);
    assert_eq!(env.current_frame_keys(), &[]);
    assert_eq!(env.current_frame_values(), &[]);
    assert_eq!(env.frame_stack(), &frame_stack);
    assert_eq!(env.bp(), bp_1);
    env.dump(&mut keys_values_out);
    assert_eq!(&keys_values_out, &keys_values);

    keys.extend_from_slice(&keys_1);
    values.extend_from_slice(&values_1);
    keys_values.clear();
    keys.iter().zip(values.iter()).for_each(|(key, value)| {
        keys_values.push((*key, *value));
    });

    keys_1.iter().zip(values_1.iter()).for_each(|(key, value)| {
        env.define(*key, *value);
    });

    assert_eq!(*env.get(keys_1[1]).unwrap(), values_1[1]);
    assert_eq!(*env.get(keys_1[0]).unwrap(), values_1[2]);

    assert_eq!(env.keys(), &keys);
    assert_eq!(env.values(), &values);
    assert_eq!(env.current_frame_keys(), &keys_1);
    assert_eq!(env.current_frame_values(), &values_1);
    assert_eq!(env.frame_stack(), &frame_stack);
    assert_eq!(env.bp(), bp_1);
    env.dump(&mut keys_values_out);
    assert_eq!(&keys_values_out, &keys_values);

    keys_1.iter().zip(values_11.iter()).for_each(|(key, value)| {
        assert!(env.set(*key, *value).is_ok());
    });

    keys_2.iter().zip(values_11.iter()).for_each(|(key, value)| {
        assert_eq!(
            env.set(*key, *value).err().unwrap(),
            ChobitEnvError::NotFound {key: *key}
        );
    });

    assert_eq!(*env.get(keys_1[0]).unwrap(), values_11[2]);
    assert_eq!(*env.get(keys_1[1]).unwrap(), values_11[1]);
    assert_eq!(*env.get(keys_1[2]).unwrap(), values_11[2]);

    values.truncate(bp_1);
    values.extend_from_slice(&values_11);
    keys_values.clear();
    keys.iter().zip(values.iter()).for_each(|(key, value)| {
        keys_values.push((*key, *value));
    });

    assert_eq!(env.keys(), &keys);
    assert_eq!(env.values(), &values);
    assert_eq!(env.current_frame_keys(), &keys_1);
    assert_eq!(env.current_frame_values(), &values_11);
    assert_eq!(env.frame_stack(), &frame_stack);
    assert_eq!(env.bp(), bp_1);
    env.dump(&mut keys_values_out);
    assert_eq!(&keys_values_out, &keys_values);

    env.push_frame();
    frame_stack.push(bp_1);

    keys_2.iter().zip(values_2.iter()).for_each(|(key, value)| {
        env.define(*key, *value);
    });

    keys.extend_from_slice(&keys_2);
    values.extend_from_slice(&values_2);
    keys_values.clear();
    keys.iter().zip(values.iter()).for_each(|(key, value)| {
        keys_values.push((*key, *value));
    });

    assert_eq!(*env.get(keys_1[0]).unwrap(), values_11[2]);
    assert_eq!(*env.get(keys_1[1]).unwrap(), values_11[1]);
    assert_eq!(*env.get(keys_1[2]).unwrap(), values_11[2]);

    assert_eq!(*env.get(keys_2[0]).unwrap(), values_2[0]);
    assert_eq!(*env.get(keys_2[1]).unwrap(), values_2[1]);

    assert_eq!(env.keys(), &keys);
    assert_eq!(env.values(), &values);
    assert_eq!(env.current_frame_keys(), &keys_2);
    assert_eq!(env.current_frame_values(), &values_2);
    assert_eq!(env.frame_stack(), &frame_stack);
    assert_eq!(env.bp(), bp_2);
    env.dump(&mut keys_values_out);
    assert_eq!(&keys_values_out, &keys_values);

    keys_2.iter().zip(values_12.iter()).for_each(|(key, value)| {
        assert!(env.set(*key, *value).is_ok());
    });

    assert_eq!(*env.get(keys_1[0]).unwrap(), values_11[2]);
    assert_eq!(*env.get(keys_1[1]).unwrap(), values_11[1]);
    assert_eq!(*env.get(keys_1[2]).unwrap(), values_11[2]);

    assert_eq!(*env.get(keys_2[0]).unwrap(), values_12[0]);
    assert_eq!(*env.get(keys_2[1]).unwrap(), values_12[1]);

    assert_eq!(
        env.set(100, 1000).err().unwrap(),
        ChobitEnvError::NotFound {key: 100}
    );

    values.truncate(bp_2);
    values.extend_from_slice(&values_12);
    keys_values.clear();
    keys.iter().zip(values.iter()).for_each(|(key, value)| {
        keys_values.push((*key, *value));
    });

    assert_eq!(env.keys(), &keys);
    assert_eq!(env.values(), &values);
    assert_eq!(env.current_frame_keys(), &keys_2);
    assert_eq!(env.current_frame_values(), &values_12);
    assert_eq!(env.frame_stack(), &frame_stack);
    assert_eq!(env.bp(), bp_2);
    env.dump(&mut keys_values_out);
    assert_eq!(&keys_values_out, &keys_values);

    let keys_values_3: Vec<(u64, i32)> = keys_3.iter().zip(values_3.iter())
        .map(|(key, value)| (*key, *value))
        .collect();

    env.store(&keys_values_3);
    frame_stack.push(bp_2);

    keys.extend_from_slice(&keys_3);
    values.extend_from_slice(&values_3);
    keys_values.clear();
    keys.iter().zip(values.iter()).for_each(|(key, value)| {
        keys_values.push((*key, *value));
    });

    assert_eq!(*env.get(keys_1[0]).unwrap(), values_11[2]);
    assert_eq!(*env.get(keys_1[1]).unwrap(), values_11[1]);
    assert_eq!(*env.get(keys_1[2]).unwrap(), values_11[2]);

    assert_eq!(*env.get(keys_2[0]).unwrap(), values_3[1]);
    assert_eq!(*env.get(keys_2[1]).unwrap(), values_12[1]);

    assert_eq!(*env.get(keys_3[0]).unwrap(), values_3[0]);
    assert_eq!(*env.get(keys_3[1]).unwrap(), values_3[1]);

    assert_eq!(
        env.set(100, 1000).err().unwrap(),
        ChobitEnvError::NotFound {key: 100}
    );

    assert_eq!(env.keys(), &keys);
    assert_eq!(env.values(), &values);
    assert_eq!(env.current_frame_keys(), &keys_3);
    assert_eq!(env.current_frame_values(), &values_3);
    assert_eq!(env.frame_stack(), &frame_stack);
    assert_eq!(env.bp(), bp_3);
    env.dump(&mut keys_values_out);
    assert_eq!(&keys_values_out, &keys_values);

    keys_3.iter().zip(values_13.iter()).for_each(|(key, value)| {
        assert!(env.set(*key, *value).is_ok());
    });

    values.truncate(bp_3);
    values.extend_from_slice(&values_13);
    keys_values.clear();
    keys.iter().zip(values.iter()).for_each(|(key, value)| {
        keys_values.push((*key, *value));
    });

    assert_eq!(*env.get(keys_1[0]).unwrap(), values_11[2]);
    assert_eq!(*env.get(keys_1[1]).unwrap(), values_11[1]);
    assert_eq!(*env.get(keys_1[2]).unwrap(), values_11[2]);

    assert_eq!(*env.get(keys_2[0]).unwrap(), values_13[1]);
    assert_eq!(*env.get(keys_2[1]).unwrap(), values_12[1]);

    assert_eq!(*env.get(keys_3[0]).unwrap(), values_13[0]);
    assert_eq!(*env.get(keys_3[1]).unwrap(), values_13[1]);

    assert_eq!(env.keys(), &keys);
    assert_eq!(env.values(), &values);
    assert_eq!(env.current_frame_keys(), &keys_3);
    assert_eq!(env.current_frame_values(), &values_13);
    assert_eq!(env.frame_stack(), &frame_stack);
    assert_eq!(env.bp(), bp_3);
    env.dump(&mut keys_values_out);
    assert_eq!(&keys_values_out, &keys_values);

    // pop_frame ----------

    assert!(env.pop_frame());
    keys.truncate(bp_3);
    values.truncate(bp_3);
    keys_values.truncate(bp_3);
    let _ = frame_stack.pop();

    assert_eq!(env.keys(), &keys);
    assert_eq!(env.values(), &values);
    assert_eq!(env.current_frame_keys(), &keys_2);
    assert_eq!(env.current_frame_values(), &values_12);
    assert_eq!(env.frame_stack(), &frame_stack);
    assert_eq!(env.bp(), bp_2);
    env.dump(&mut keys_values_out);
    assert_eq!(&keys_values_out, &keys_values);

    assert!(env.pop_frame());
    keys.truncate(bp_2);
    values.truncate(bp_2);
    keys_values.truncate(bp_2);
    let _ = frame_stack.pop();

    assert_eq!(env.keys(), &keys);
    assert_eq!(env.values(), &values);
    assert_eq!(env.current_frame_keys(), &keys_1);
    assert_eq!(env.current_frame_values(), &values_11);
    assert_eq!(env.frame_stack(), &frame_stack);
    assert_eq!(env.bp(), bp_1);
    env.dump(&mut keys_values_out);
    assert_eq!(&keys_values_out, &keys_values);

    assert!(env.pop_frame());
    keys.truncate(bp_1);
    values.truncate(bp_1);
    keys_values.truncate(bp_1);
    let _ = frame_stack.pop();

    assert_eq!(env.keys(), &keys);
    assert_eq!(env.values(), &values);
    assert_eq!(env.current_frame_keys(), &[]);
    assert_eq!(env.current_frame_values(), &[]);
    assert_eq!(env.frame_stack(), &frame_stack);
    assert_eq!(env.bp(), bp_1);
    env.dump(&mut keys_values_out);
    assert_eq!(&keys_values_out, &keys_values);

    assert!(!env.pop_frame());
}
