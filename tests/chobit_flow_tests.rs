extern crate chobitlibs;

use std::{
    prelude::rust_2021::*,
    fmt
};

use chobitlibs::chobit_flow::*;

#[derive(Debug, Clone, PartialEq)]
enum TestError {
    PrintError,
    ConstError,
    HubError,
    AddError,
    StopError
}

impl OperatorError for TestError {
    fn write_one_line_json(
        &self,
        formatter: &mut fmt::Formatter
    ) -> fmt::Result {
        match self {
            TestError::PrintError =>
                write!(formatter, r#"{{"error": "PrintError"}}"#),
            TestError::ConstError =>
                write!(formatter, r#"{{"error": "ConstError"}}"#),
            TestError::HubError =>
                write!(formatter, r#"{{"error": "HubError"}}"#),
            TestError::AddError =>
                write!(formatter, r#"{{"error": "AddError"}}"#),
            TestError::StopError =>
                write!(formatter, r#"{{"error": "StopError"}}"#),
        }
    }
}

struct Print;
impl Operator<i32> for Print {
    fn receive(
        &mut self,
        inlets: &[Option<i32>],
        outlets: &mut [Option<i32>]
    ) -> Result<OperatorCommand<i32>, Box<dyn OperatorError>> {
        println!("Print: {:?}", inlets.get(0).unwrap().as_ref().unwrap());

        if let (Some(inlet), Some(outlet)) = (
            inlets.get(0),
            outlets.get_mut(0)
        ) {
            *outlet = inlet.clone();

            Ok(OperatorCommand::Go)
        } else {
            Err(Box::new(TestError::PrintError))
        }
    }

    fn resume(
        &mut self,
        _data: Option<i32>,
        _outlets: &mut [Option<i32>]
    ) -> Result<OperatorCommand<i32>, Box<dyn OperatorError>> {
        Ok(OperatorCommand::Go)
    }
}

struct Const {
    data: i32
}

impl Operator<i32> for Const {
    fn receive(
        &mut self,
        _inlets: &[Option<i32>],
        outlets: &mut [Option<i32>]
    ) -> Result<OperatorCommand<i32>, Box<dyn OperatorError>> {
        println!("Const {}", self.data);

        if let Some(outlet) = outlets.get_mut(0) {
            *outlet = Some(self.data);

            Ok(OperatorCommand::Go)
        } else {
            Err(Box::new(TestError::ConstError))
        }
    }

    fn resume(
        &mut self,
        _data: Option<i32>,
        _outlets: &mut [Option<i32>]
    ) -> Result<OperatorCommand<i32>, Box<dyn OperatorError>> {
        Ok(OperatorCommand::Go)
    }
}

struct Hub;

impl Operator<i32> for Hub {
    fn receive(
        &mut self,
        inlets: &[Option<i32>],
        outlets: &mut [Option<i32>]
    ) -> Result<OperatorCommand<i32>, Box<dyn OperatorError>> {
        println!("Hub!");

        if let Some(inlet) = inlets.get(0) {
            outlets.iter_mut().for_each(|outlet| {*outlet = inlet.clone();});

            Ok(OperatorCommand::Go)
        } else {
            Err(Box::new(TestError::HubError))
        }
    }

    fn resume(
        &mut self,
        _data: Option<i32>,
        _outlets: &mut [Option<i32>]
    ) -> Result<OperatorCommand<i32>, Box<dyn OperatorError>> {
        Ok(OperatorCommand::Go)
    }
}

struct Add;

impl Operator<i32> for Add {
    fn receive(
        &mut self,
        inlets: &[Option<i32>],
        outlets: &mut [Option<i32>]
    ) -> Result<OperatorCommand<i32>, Box<dyn OperatorError>> {
        println!("Add!");

        if let (
            Some(Some(x)),
            Some(inlet_2),
            Some(outlet)
        ) = (
            inlets.get(0),
            inlets.get(1),
            outlets.get_mut(0)
        ) {
            let y = if let Some(y) = inlet_2 {
                *y
            } else {
                0
            };

            *outlet = Some(*x + y);

            Ok(OperatorCommand::Go)
        } else {
            Err(Box::new(TestError::AddError))
        }
    }

    fn resume(
        &mut self,
        _data: Option<i32>,
        _outlets: &mut [Option<i32>]
    ) -> Result<OperatorCommand<i32>, Box<dyn OperatorError>> {
        Ok(OperatorCommand::Go)
    }
}

struct Stop;

impl Operator<i32> for Stop {
    fn receive(
        &mut self,
        inlets: &[Option<i32>],
        _outlets: &mut [Option<i32>]
    ) -> Result<OperatorCommand<i32>, Box<dyn OperatorError>> {
        println!("Stop!");

        if let Some(inlet) = inlets.get(0) {
            Ok(OperatorCommand::Stop(inlet.clone()))
        } else {
            Err(Box::new(TestError::StopError))
        }
    }

    fn resume(
        &mut self,
        data: Option<i32>,
        outlets: &mut [Option<i32>]
    ) -> Result<OperatorCommand<i32>, Box<dyn OperatorError>> {
        println!("Resume!");

        if let Some(outlet) = outlets.get_mut(0) {
            *outlet = data;

            Ok(OperatorCommand::Go)
        } else {
            Err(Box::new(TestError::StopError))
        }
    }
}

struct Test1;

impl Operator<i32> for Test1 {
    fn receive(
        &mut self,
        inlets: &[Option<i32>],
        outlets: &mut [Option<i32>]
    ) -> Result<OperatorCommand<i32>, Box<dyn OperatorError>> {
        println!("Test1::receive()");

        inlets.iter().for_each(|inlet| {
            match &inlet {
                Some(data) => {println!("{}", data);},
                None => {println!("None");},
            }
        });

        outlets.iter_mut().for_each(|outlet| {*outlet = Some(0);});

        Ok(OperatorCommand::Go)
    }

    fn resume(
        &mut self,
        data: Option<i32>,
        outlets: &mut [Option<i32>]
    ) -> Result<OperatorCommand<i32>, Box<dyn OperatorError>> {
        println!("Test1::resume()");

        match data {
            Some(data) => {println!("{}", data);},
            None => {println!("None");},
        }

        outlets.iter_mut().for_each(|outlet| {*outlet = Some(0);});

        Ok(OperatorCommand::Go)
    }
}


fn gen_const_tree() -> (
    ChobitFlow<i32>,
    u64,
    u64,
    u64,
    u64,
    u64,
    u64,
    u64,
    u64,
    u64,
    u64,
) {
    let mut cf = ChobitFlow::<i32>::new(64);

    let const_1_id =
        cf.add_node(Box::new(Const {data: 1}), 1, 1).unwrap();
    let const_2_id =
        cf.add_node(Box::new(Const {data: 2}), 1, 1).unwrap();
    let const_3_id =
        cf.add_node(Box::new(Const {data: 3}), 1, 1).unwrap();
    let const_4_id =
        cf.add_node(Box::new(Const {data: 4}), 1, 1).unwrap();
    let const_5_id =
        cf.add_node(Box::new(Const {data: 5}), 1, 1).unwrap();
    let const_6_id =
        cf.add_node(Box::new(Const {data: 6}), 1, 1).unwrap();
    let const_7_id =
        cf.add_node(Box::new(Const {data: 7}), 1, 1).unwrap();

    let hub_1_id = cf.add_node(Box::new(Hub), 1, 2).unwrap();
    let hub_2_id = cf.add_node(Box::new(Hub), 1, 2).unwrap();
    let hub_3_id = cf.add_node(Box::new(Hub), 1, 2).unwrap();

    assert!(cf.connect_nodes(const_1_id, 0, hub_1_id, 0).is_ok());
    assert!(cf.connect_nodes(hub_1_id, 0, const_2_id, 0).is_ok());
    assert!(cf.connect_nodes(hub_1_id, 1, const_5_id, 0).is_ok());
    assert!(cf.connect_nodes(const_2_id, 0, hub_2_id, 0).is_ok());
    assert!(cf.connect_nodes(hub_2_id, 0, const_3_id, 0).is_ok());
    assert!(cf.connect_nodes(hub_2_id, 1, const_4_id, 0).is_ok());
    assert!(cf.connect_nodes(const_5_id, 0, hub_3_id, 0).is_ok());
    assert!(cf.connect_nodes(hub_3_id, 0, const_6_id, 0).is_ok());
    assert!(cf.connect_nodes(hub_3_id, 1, const_7_id, 0).is_ok());

    (
        cf,
        const_1_id,
        const_2_id,
        const_3_id,
        const_4_id,
        const_5_id,
        const_6_id,
        const_7_id,
        hub_1_id,
        hub_2_id,
        hub_3_id,
    )
}

#[test]
fn print_test() {
    let mut cf = ChobitFlow::<i32>::new(64);
    let print_id = cf.add_node(Box::new(Print), 1, 1).unwrap();

    assert_eq!(
        cf.send(None, print_id, 0, 99).unwrap(),
        ChobitFlowResult::Ended
    );
}

#[test]
fn connect_one_test() {
    let mut cf = ChobitFlow::<i32>::new(64);
    let print_id = cf.add_node(Box::new(Print), 1, 1).unwrap();
    let const_1_id =
        cf.add_node(Box::new(Const {data: 1}), 1, 1).unwrap();

    assert!(cf.connect_nodes(const_1_id, 0, print_id, 0).is_ok());

    assert_eq!(
        cf.send(None, const_1_id, 0, 0).unwrap(),
        ChobitFlowResult::Ended
    );
}

#[test]
fn connect_tree_test() {
    let (mut cf, const_1_id, ..) = gen_const_tree();

    assert_eq!(
        cf.send(None, const_1_id, 0, 0).unwrap(),
        ChobitFlowResult::Ended
    );
}

#[test]
fn add_test() {
    let mut cf = ChobitFlow::<i32>::new(64);
    let print_id = cf.add_node(Box::new(Print), 1, 1).unwrap();

    let const_1_id =
        cf.add_node(Box::new(Const {data: 1}), 1, 1).unwrap();
    let const_2_id =
        cf.add_node(Box::new(Const {data: 2}), 1, 1).unwrap();

    let add_id = cf.add_node(Box::new(Add), 2, 1).unwrap();

    assert!(cf.connect_nodes(const_1_id, 0, add_id, 0).is_ok());
    assert!(cf.connect_nodes(const_2_id, 0, add_id, 1).is_ok());
    assert!(cf.connect_nodes(add_id, 0, print_id, 0).is_ok());

    assert_eq!(
        cf.send(None, const_1_id, 0, 0).unwrap(),
        ChobitFlowResult::Ended
    );

    println!("-----");

    assert_eq!(
        cf.send(None, const_2_id, 0, 0).unwrap(),
        ChobitFlowResult::Ended
    );


    assert_eq!(
        cf.send(None, const_1_id, 0, 0).unwrap(),
        ChobitFlowResult::Ended
    );
}

#[test]
fn stop_test() {
    let (
        mut cf,
        const_1_id,
        const_2_id,
        _const_3_id,
        _const_4_id,
        const_5_id,
        _const_6_id,
        _const_7_id,
        hub_1_id,
        _hub_2_id,
        _hub_3_id,
    ) = gen_const_tree();

    let stop_1_id = cf.add_node(Box::new(Stop), 1, 1).unwrap();
    let stop_2_id = cf.add_node(Box::new(Stop), 1, 1).unwrap();

    assert!(cf.disconnect_nodes(hub_1_id, 0, const_2_id, 0).is_ok());
    assert!(cf.disconnect_nodes(hub_1_id, 1, const_5_id, 0).is_ok());

    assert!(cf.connect_nodes(hub_1_id, 0, stop_1_id, 0).is_ok());
    assert!(cf.connect_nodes(stop_1_id, 0, const_2_id, 0).is_ok());

    assert!(cf.connect_nodes(hub_1_id, 1, stop_2_id, 0).is_ok());
    assert!(cf.connect_nodes(stop_2_id, 0, const_5_id, 0).is_ok());

    assert_eq!(
        cf.send(None, const_1_id, 0, 0).unwrap(),
        ChobitFlowResult::Stopped(stop_1_id)
    );

    println!("-----");

    assert_eq!(
        cf.resume().unwrap(),
        ChobitFlowResult::Stopped(stop_2_id)
    );

    println!("-----");

    assert_eq!(
        cf.resume().unwrap(),
        ChobitFlowResult::Ended
    );
}

#[test]
fn connect_error_test() {
    let mut cf = ChobitFlow::new(64);

    let const_1_id =
        cf.add_node(Box::new(Const {data: 1}), 1, 1).unwrap();

    let test_1_1 = cf.add_node(Box::new(Test1), 1, 1).unwrap();

    match cf.connect_nodes(const_1_id, 0, test_1_1, 1) {
        Err(ChobitFlowError::InletNotFound {id, inlet_pos}) => {
            assert_eq!(id, test_1_1);
            assert_eq!(inlet_pos, 1);
        },

        other => {panic!("{:?}", other);}
    }
}
