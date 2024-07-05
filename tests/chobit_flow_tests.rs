extern crate chobitlibs;

use std::{
    prelude::rust_2021::*,
    cell::RefCell,
    rc::Rc
};

use chobitlibs::{chobit_flow::*, chobit_map::*};

struct Message {
    pub direction: Direction,
    pub route: Vec<u64>,
    pub senders: Vec<Option<u64>>
}

enum Direction {
    Left,
    Right
}

const ID_START: u64 = 1;
const ID_LEFT_1: u64 = 2;
const ID_RIGHT_1: u64 = 3;
const ID_MIDDLE: u64 = 4;
const ID_LEFT_2: u64 = 5;
const ID_RIGHT_2: u64 = 6;
const ID_END: u64 = 7;

struct NodeStart;
struct NodeLeft1;
struct NodeRight1;
struct NodeMiddle;
struct NodeLeft2;
struct NodeRight2;
struct NodeEnd;

impl Node for NodeStart {
    type Message = Message;

    fn id(&self) -> u64 {ID_START}

    fn execute(
        &mut self,
        prev_node: Option<u64>,
        mut message: Message
    ) -> (Option<u64>, Message) {
        message.route.push(self.id());
        message.senders.push(prev_node);

        let next_id = match message.direction {
            Direction::Left => ID_LEFT_1,
            Direction::Right => ID_RIGHT_1
        };

        (Some(next_id), message)
    }
}

impl Node for NodeLeft1 {
    type Message = Message;

    fn id(&self) -> u64 {ID_LEFT_1}

    fn execute(
        &mut self,
        prev_node: Option<u64>,
        mut message: Message
    ) -> (Option<u64>, Message) {
        message.route.push(self.id());
        message.senders.push(prev_node);

        (Some(ID_MIDDLE), message)
    }
}

impl Node for NodeRight1 {
    type Message = Message;

    fn id(&self) -> u64 {ID_RIGHT_1}

    fn execute(
        &mut self,
        prev_node: Option<u64>,
        mut message: Message
    ) -> (Option<u64>, Message) {
        message.route.push(self.id());
        message.senders.push(prev_node);

        (Some(ID_MIDDLE), message)
    }
}

impl Node for NodeMiddle {
    type Message = Message;

    fn id(&self) -> u64 {ID_MIDDLE}

    fn execute(
        &mut self,
        prev_node: Option<u64>,
        mut message: Message
    ) -> (Option<u64>, Message) {
        message.route.push(self.id());
        message.senders.push(prev_node);

        let next_id = match message.direction {
            Direction::Left => ID_LEFT_2,
            Direction::Right => ID_RIGHT_2
        };

        (Some(next_id), message)
    }
}

impl Node for NodeLeft2 {
    type Message = Message;

    fn id(&self) -> u64 {ID_LEFT_2}

    fn execute(
        &mut self,
        prev_node: Option<u64>,
        mut message: Message
    ) -> (Option<u64>, Message) {
        message.route.push(self.id());
        message.senders.push(prev_node);

        (Some(ID_END), message)
    }
}

impl Node for NodeRight2 {
    type Message = Message;

    fn id(&self) -> u64 {ID_RIGHT_2}

    fn execute(
        &mut self,
        prev_node: Option<u64>,
        mut message: Message
    ) -> (Option<u64>, Message) {
        message.route.push(self.id());
        message.senders.push(prev_node);

        (Some(ID_END), message)
    }
}

impl Node for NodeEnd {
    type Message = Message;

    fn id(&self) -> u64 {ID_END}

    fn execute(
        &mut self,
        prev_node: Option<u64>,
        mut message: Message
    ) -> (Option<u64>, Message) {
        message.route.push(self.id());
        message.senders.push(prev_node);

        (None, message)
    }
}

struct TestIdToNode {
    pub map: ChobitMap::<Rc<RefCell<dyn Node<Message = Message>>>>
}

impl IdToNode for TestIdToNode {
    type Message = Message;

    #[inline]
    fn id_to_node(
        &mut self,
        id: u64
    ) -> Option<Rc<RefCell<dyn Node<Message = Message>>>> {
        self.map.get(id).cloned()
    }
}

fn gen_map() -> ChobitMap<Rc<RefCell<dyn Node<Message = Message>>>> {
    let mut ret =
        ChobitMap::<Rc<RefCell<dyn Node<Message = Message>>>>::new(32);

    assert!(ret.add(ID_START, Rc::new(RefCell::new(NodeStart))).is_ok());
    assert!(ret.add(ID_LEFT_1, Rc::new(RefCell::new(NodeLeft1))).is_ok());
    assert!(ret.add(ID_LEFT_2, Rc::new(RefCell::new(NodeLeft2))).is_ok());
    assert!(ret.add(ID_RIGHT_1, Rc::new(RefCell::new(NodeRight1))).is_ok());
    assert!(ret.add(ID_RIGHT_2, Rc::new(RefCell::new(NodeRight2))).is_ok());
    assert!(ret.add(ID_MIDDLE, Rc::new(RefCell::new(NodeMiddle))).is_ok());
    assert!(ret.add(ID_END, Rc::new(RefCell::new(NodeEnd))).is_ok());

    ret
}

fn gen_chobit_flow() -> ChobitFlow<Message, TestIdToNode> {
    let map = gen_map();

    let id_to_node = TestIdToNode {map: map};

    ChobitFlow::<Message, TestIdToNode>::new(
        ID_START,
        id_to_node
    )
}

fn gen_message(direction: Direction) -> Message {
    Message {
        direction: direction,
        route: Vec::<u64>::new(),
        senders: Vec::<Option<u64>>::new()
    }
}

#[test]
fn chobit_flow_test_1() {
    let message = gen_message(Direction::Left);

    let mut graph = gen_chobit_flow();

    let message = {
        assert_eq!(graph.next_id(), ID_START);
        let (opt_next_id, message) = graph.next(None, message).unwrap();
        assert_eq!(opt_next_id, Some(ID_LEFT_1));

        assert_eq!(graph.next_id(), ID_LEFT_1);
        let (opt_next_id, message) =
            graph.next(Some(ID_START), message).unwrap();
        assert_eq!(opt_next_id, Some(ID_MIDDLE));

        assert_eq!(graph.next_id(), ID_MIDDLE);
        let (opt_next_id, message) =
            graph.next(Some(ID_LEFT_1), message).unwrap();
        assert_eq!(opt_next_id, Some(ID_LEFT_2));

        assert_eq!(graph.next_id(), ID_LEFT_2);
        let (opt_next_id, message) =
            graph.next(Some(ID_MIDDLE), message).unwrap();
        assert_eq!(opt_next_id, Some(ID_END));

        assert_eq!(graph.next_id(), ID_END);
        let (opt_next_id, message) =
            graph.next(Some(ID_LEFT_2), message).unwrap();
        assert_eq!(opt_next_id, None);

        message
    };

    assert_eq!(
        message.route.as_slice(),
        &[
            ID_START,
            ID_LEFT_1,
            ID_MIDDLE,
            ID_LEFT_2,
            ID_END
        ]
    );

    assert_eq!(
        message.senders.as_slice(),
        &[
            None,
            Some(ID_START),
            Some(ID_LEFT_1),
            Some(ID_MIDDLE),
            Some(ID_LEFT_2)
        ]
    );
}

#[test]
fn chobit_flow_test_2() {
    let message = gen_message(Direction::Right);

    let mut graph = gen_chobit_flow();

    let message = {
        assert_eq!(graph.next_id(), ID_START);
        let (opt_next_id, message) = graph.next(None, message).unwrap();
        assert_eq!(opt_next_id, Some(ID_RIGHT_1));

        assert_eq!(graph.next_id(), ID_RIGHT_1);
        let (opt_next_id, message) =
            graph.next(Some(ID_START), message).unwrap();
        assert_eq!(opt_next_id, Some(ID_MIDDLE));

        assert_eq!(graph.next_id(), ID_MIDDLE);
        let (opt_next_id, message) =
            graph.next(Some(ID_RIGHT_1), message).unwrap();
        assert_eq!(opt_next_id, Some(ID_RIGHT_2));

        assert_eq!(graph.next_id(), ID_RIGHT_2);
        let (opt_next_id, message) =
            graph.next(Some(ID_MIDDLE), message).unwrap();
        assert_eq!(opt_next_id, Some(ID_END));

        assert_eq!(graph.next_id(), ID_END);
        let (opt_next_id, message) =
            graph.next(Some(ID_RIGHT_2), message).unwrap();
        assert_eq!(opt_next_id, None);

        message
    };

    assert_eq!(
        message.route.as_slice(),
        &[
            ID_START,
            ID_RIGHT_1,
            ID_MIDDLE,
            ID_RIGHT_2,
            ID_END
        ]
    );

    assert_eq!(
        message.senders.as_slice(),
        &[
            None,
            Some(ID_START),
            Some(ID_RIGHT_1),
            Some(ID_MIDDLE),
            Some(ID_RIGHT_2)
        ]
    );
}

#[test]
fn chobit_flow_test_3() {
    let message = gen_message(Direction::Left);

    let mut graph = gen_chobit_flow();

    let message = graph.run(None, message).unwrap();

    assert_eq!(
        message.route.as_slice(),
        &[
            ID_START,
            ID_LEFT_1,
            ID_MIDDLE,
            ID_LEFT_2,
            ID_END
        ]
    );

    assert_eq!(
        message.senders.as_slice(),
        &[
            None,
            Some(ID_START),
            Some(ID_LEFT_1),
            Some(ID_MIDDLE),
            Some(ID_LEFT_2)
        ]
    );
}

#[test]
fn chobit_flow_test_4() {
    let message = gen_message(Direction::Right);

    let mut graph = gen_chobit_flow();

    let message = graph.run(None, message).unwrap();

    assert_eq!(
        message.route.as_slice(),
        &[
            ID_START,
            ID_RIGHT_1,
            ID_MIDDLE,
            ID_RIGHT_2,
            ID_END
        ]
    );

    assert_eq!(
        message.senders.as_slice(),
        &[
            None,
            Some(ID_START),
            Some(ID_RIGHT_1),
            Some(ID_MIDDLE),
            Some(ID_RIGHT_2)
        ]
    );
}

const ERROR_ID: u64 = ID_END + 10;

struct NodeLeft3;
impl Node for NodeLeft3 {
    type Message = Message;

    fn id(&self) -> u64 {ID_LEFT_2}

    fn execute(
        &mut self,
        prev_node: Option<u64>,
        mut message: Message
    ) -> (Option<u64>, Message) {
        message.route.push(self.id());
        message.senders.push(prev_node);

        (Some(ERROR_ID), message)
    }
}

#[test]
fn node_not_found_test_1() {
    let mut map = gen_map();
    *(map.get_mut(ID_LEFT_2).unwrap()) = Rc::new(RefCell::new(NodeLeft3));

    let id_to_node = TestIdToNode {map: map};

    let mut graph = ChobitFlow::<Message, TestIdToNode>::new(
        ID_START,
        id_to_node
    );

    let message = gen_message(Direction::Left);

    assert_eq!(graph.next_id(), ID_START);
    let (opt_next_id, message) = graph.next(None, message).unwrap();
    assert_eq!(opt_next_id, Some(ID_LEFT_1));

    assert_eq!(graph.next_id(), ID_LEFT_1);
    let (opt_next_id, message) = graph.next(Some(ID_START), message).unwrap();
    assert_eq!(opt_next_id, Some(ID_MIDDLE));

    assert_eq!(graph.next_id(), ID_MIDDLE);
    let (opt_next_id, message) = graph.next(Some(ID_LEFT_1), message).unwrap();
    assert_eq!(opt_next_id, Some(ID_LEFT_2));

    assert_eq!(graph.next_id(), ID_LEFT_2);
    let (opt_next_id, message) = graph.next(Some(ID_MIDDLE), message).unwrap();
    assert_eq!(opt_next_id, Some(ERROR_ID));

    assert_eq!(graph.next_id(), ERROR_ID);
    let result = graph.next(Some(ID_LEFT_1), message).err().unwrap();
    assert_eq!(result, ChobitFlowError::NodeNotFound {id: ERROR_ID});

    println!("{result}");
}

#[test]
fn node_not_found_test_2() {
    let mut map = gen_map();
    *(map.get_mut(ID_LEFT_2).unwrap()) = Rc::new(RefCell::new(NodeLeft3));

    let id_to_node = TestIdToNode {map: map};

    let mut graph = ChobitFlow::<Message, TestIdToNode>::new(
        ID_START,
        id_to_node
    );

    let message = gen_message(Direction::Left);
    assert_eq!(
        graph.run(None, message).err().unwrap(),
        ChobitFlowError::NodeNotFound {id: ERROR_ID}
    );
}

#[test]
fn failed_to_borrow_test_1() {
    let map = gen_map();
    let node = map.get(ID_END).unwrap().clone();
    let _node = node.borrow();

    let id_to_node = TestIdToNode {map: map};

    let mut graph = ChobitFlow::<Message, TestIdToNode>::new(
        ID_START,
        id_to_node
    );

    let message = gen_message(Direction::Left);

    assert_eq!(graph.next_id(), ID_START);
    let (opt_next_id, message) = graph.next(None, message).unwrap();
    assert_eq!(opt_next_id, Some(ID_LEFT_1));

    assert_eq!(graph.next_id(), ID_LEFT_1);
    let (opt_next_id, message) = graph.next(Some(ID_START), message).unwrap();
    assert_eq!(opt_next_id, Some(ID_MIDDLE));

    assert_eq!(graph.next_id(), ID_MIDDLE);
    let (opt_next_id, message) = graph.next(Some(ID_LEFT_1), message).unwrap();
    assert_eq!(opt_next_id, Some(ID_LEFT_2));

    assert_eq!(graph.next_id(), ID_LEFT_2);
    let (opt_next_id, message) = graph.next(Some(ID_MIDDLE), message).unwrap();
    assert_eq!(opt_next_id, Some(ID_END));

    assert_eq!(graph.next_id(), ID_END);
    let result = graph.next(Some(ID_LEFT_2), message).err().unwrap();
    assert_eq!(result, ChobitFlowError::FailedToBorrowNode {id: ID_END});

    println!("{result}");
}

#[test]
fn failed_to_borrow_test_2() {
    let map = gen_map();
    let node = map.get(ID_END).unwrap().clone();
    let _node = node.borrow();

    let id_to_node = TestIdToNode {map: map};

    let mut graph = ChobitFlow::<Message, TestIdToNode>::new(
        ID_START,
        id_to_node
    );

    let message = gen_message(Direction::Left);

    assert_eq!(
        graph.run(None, message).err().unwrap(),
        ChobitFlowError::FailedToBorrowNode {id: ID_END}
    );
}
