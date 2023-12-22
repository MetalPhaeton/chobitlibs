//        DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004 
//
// Copyright (C) 2023 Hironori Ishibashi
//
// Everyone is permitted to copy and distribute verbatim or modified 
// copies of this license document, and changing it is allowed as long 
// as the name is changed. 
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE 
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION 
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use alloc::{rc::Rc, vec::Vec, boxed::Box};
use core::{cell::RefCell, fmt};

pub trait OperatorError : fmt::Debug {
    fn write_one_line_json(
        &self,
        formatter: &mut fmt::Formatter
    ) -> fmt::Result;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    Bang,
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    ISize(isize),
    USize(usize),
    Bool(bool),
    Char(char),
    Bytes(Rc<RefCell<Vec<u8>>>),
    String(Rc<RefCell<Vec<u8>>>),
    Cons(Box<Data>, Box<Data>)
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperatorCommand<'a> {
    Go(&'a [Option<Data>]),
    Stop(Option<Data>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Inlet {
    sender: Option<u64>,
    data: Option<Data>
}

impl Inlet {
    #[inline]
    pub fn data(&self) -> Option<&Data> {
        self.data.as_ref()
    }
}

pub trait Operator {
    fn receive(
        &mut self,
        inlets: &[Inlet]
    ) -> Result<OperatorCommand, Box<dyn OperatorError>>;

    fn resume(
        &mut self,
        data: Option<Data>
    ) -> Result<OperatorCommand, Box<dyn OperatorError>>;
}

pub enum GraphError {
    Operator(Box<dyn OperatorError>),
    NumberOfOutletsIsWrong {wrong: usize, correct: usize},
    NodeNotFound {id: u64},
    OutletNotFound {id: u64, outlet_pos: usize},
    InletNotFound {id: u64, inlet_pos: usize},
    ReceiverNotFound {id: u64, inlet_pos: usize},
    NoStandbyNode
}

#[derive(Debug, Clone, PartialEq)]
enum GraphCommand {
    Continue(u64),
    Send(u64, usize, Data),  // (id, inlet position, data)
    Ended,
    Stopped(u64)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChobitFlowResult {
    Ended,
    Stopped(u64)
}

#[derive(Debug, Clone, PartialEq)]
struct Outlet {
    receivers: Vec<(u64, usize)>,  // (id, inlet position)
    data: Option<Data>
}

struct Node {
    operator: Box<dyn Operator>,

    id: u64,
    inlets: Box<[Inlet]>,
    outlets: Box<[Outlet]>,

    standby_data: Option<Data>,
    current_outlet: usize,
    current_receiver: usize
}

macro_rules! handle_operator_result {
    ($self:expr, $result:expr) => {
        match $result {
            Ok(OperatorCommand::Go(outputs)) => {
                if outputs.len() == (*$self.outlets).len() {
                    outputs.iter().zip((*$self.outlets).iter_mut()).for_each(
                        |(output, outlet)| {outlet.data = output.clone();}
                    );

                    $self.continue_output()
                } else {
                    Err(GraphError::NumberOfOutletsIsWrong {
                        wrong: outputs.len(),
                        correct: (*$self.outlets).len()
                    })
                }
            },

            Ok(OperatorCommand::Stop(data)) => {
                $self.standby_data = data;
                Ok(GraphCommand::Stopped($self.id))
            },

            Err(error) => Err(GraphError::Operator(error))
        }
    };
}

impl Node {
    fn receive_hot_inlet(&mut self) -> Result<GraphCommand, GraphError> {
        self.current_outlet = 0;
        self.current_receiver = 0;

        handle_operator_result!(self, self.operator.receive(&*self.inlets))
    }

    fn resume(&mut self) -> Result<GraphCommand, GraphError> {
        self.current_outlet = 0;
        self.current_receiver = 0;

        handle_operator_result!(
            self,
            self.operator.resume(self.standby_data.take())
        )
    }

    fn continue_output(&mut self) -> Result<GraphCommand, GraphError> {
        match (*self.outlets).get(self.current_outlet) {
            Some(outlet) => match outlet.receivers.get(self.current_receiver) {
                Some((receiver_id, inlet_pos)) => {
                    self.current_receiver += 1;

                    match &outlet.data {
                        Some(data) => Ok(GraphCommand::Send(
                            *receiver_id,
                            *inlet_pos,
                            data.clone()
                        )),

                        None => self.continue_output()
                    }
                },

                None => {
                    self.current_outlet += 1;
                    self.current_receiver = 0;

                    self.continue_output()
                }
            },

            None => match (*self.inlets).get(0) {
                Some(inlet) => match inlet.sender {
                    Some(sender) => Ok(GraphCommand::Continue(sender)),

                    None => Ok(GraphCommand::Ended)
                },

                None => Ok(GraphCommand::Ended)
            }
        }
    }
}

pub struct ChobitFlow {
    id_table: Vec<Vec<u64>>,
    node_table: Vec<Vec<Node>>,

    id_mask: u64,
    id_candidate: u64,

    standby_node_id: Option<u64>
}

macro_rules! handle_graph_command {
    ($self:expr, $command:expr) => {
        match $command {
            GraphCommand::Continue(next_id) => $self.continue_output(next_id),

            GraphCommand::Send(next_id, next_inlet_pos, data) =>
                $self.send(next_id, next_inlet_pos, data),

            GraphCommand::Ended => Ok(ChobitFlowResult::Ended),

            GraphCommand::Stopped(id) => Ok(ChobitFlowResult::Stopped(id))
        }
    };
}

impl ChobitFlow {
    pub fn new(table_size: usize) -> Self {
        let table_size = Self::check_table_size(table_size);

        Self {
            id_table: Self::init_id_table(table_size),
            node_table: Self::init_node_table(table_size),

            id_mask: Self::init_id_mask(table_size),
            id_candidate: 0,

            standby_node_id: None
        }
    }

    fn check_table_size(table_size: usize) -> usize {
        const MASK_1: u64 = 0xffffffff00000000;
        const MASK_2: u64 = 0xffff0000ffff0000;
        const MASK_3: u64 = 0xff00ff00ff00ff00;
        const MASK_4: u64 = 0xf0f0f0f0f0f0f0f0;
        const MASK_5: u64 = 0xcccccccccccccccc;
        const MASK_6: u64 = 0xaaaaaaaaaaaaaaaa;

        macro_rules! core {
            ($variable:expr, $mask:expr) => {
                match $variable & $mask {
                    0u64 => $variable,
                    masked_variable => masked_variable
                }
            };
        }

        let size = table_size as u64;

        let size = core!(size, MASK_1);
        let size = core!(size, MASK_2);
        let size = core!(size, MASK_3);
        let size = core!(size, MASK_4);
        let size = core!(size, MASK_5);
        let size = core!(size, MASK_6);

        match size as usize{
            0 => 1usize,
            ret => ret
        }
    }

    #[inline]
    fn init_id_table(table_size: usize) -> Vec<Vec<u64>> {
        let mut ret = Vec::<Vec<u64>>::with_capacity(table_size);

        for _ in 0..table_size {
            ret.push(Vec::<u64>::new());
        }

        ret
    }

    #[inline]
    fn init_node_table(table_size: usize) -> Vec<Vec<Node>> {
        let mut ret = Vec::<Vec<Node>>::with_capacity(table_size);

        for _ in 0..table_size {
            ret.push(Vec::<Node>::new());
        }

        ret
    }

    #[inline]
    fn init_id_mask(table_size: usize) -> u64 {
        (table_size as u64) - 1
    }

    pub fn add_node(
        &mut self,
        operator: Box<dyn Operator>,
        inlet_size: usize,
        outlet_size: usize
    ) -> Result<u64, GraphError> {
        let id = self.id_candidate;
        self.id_candidate += 1;

        let table_index = (id & self.id_mask) as usize;

        let id_vec = &mut self.id_table[table_index];

        match id_vec.binary_search(&id) {
            // try again
            Ok(..) => self.add_node(operator, inlet_size, outlet_size),

            Err(record_index) => {
                id_vec.insert(record_index, id);
                self.node_table[table_index].insert(
                    record_index,
                    Node {
                        operator: operator,
                        id: id,
                        inlets: {
                            let mut ret =
                                Vec::<Inlet>::with_capacity(inlet_size);

                            for _ in 0..inlet_size {
                                ret.push(Inlet {
                                    sender: None,
                                    data: None
                                })
                            }

                            ret.into_boxed_slice()
                        },
                        outlets: {
                            let mut ret =
                                Vec::<Outlet>::with_capacity(outlet_size);

                            for _ in 0..outlet_size {
                                ret.push(Outlet {
                                    receivers: Vec::<(u64, usize)>::new(),
                                    data: None
                                })
                            }

                            ret.into_boxed_slice()
                        },

                        standby_data: None,
                        current_outlet: 0,
                        current_receiver: 0
                    }
                );

                Ok(id)
            }
        }
    }

    pub fn remove_node(&mut self, id: u64) -> Result<(), GraphError> {
        let table_index = (id & self.id_mask) as usize;

        let id_vec = &mut self.id_table[table_index];

        match id_vec.binary_search(&id) {
            Ok(record_index) => {
                id_vec.remove(record_index);
                Ok(())
            },

            Err(..) => Err(GraphError::NodeNotFound {id: id})
        }
    }

    #[inline]
    fn get_index(&self, id: u64) -> Option<(usize, usize)> {
        let table_index = (id & self.id_mask) as usize;

        match self.id_table[table_index].binary_search(&id) {
            Ok(record_index) => Some((table_index, record_index)),

            Err(..) => None
        }
    }

    pub fn connect_nodes(
        &mut self,
        sender_id: u64,
        sender_outlet_pos: usize,
        receiver_id: u64,
        receiver_inlet_pos: usize
    ) -> Result<(), GraphError> {
        // check receiver.
        let _ = self.get_index(receiver_id).ok_or_else(
            || GraphError::NodeNotFound {id: receiver_id}
        )?;

        let sender = {
            let (table_index, record_index) =
                self.get_index(sender_id).ok_or_else(
                    || GraphError::NodeNotFound {id: sender_id}
                )?;

            &mut self.node_table[table_index][record_index]
        };

        match (*sender.outlets).get_mut(sender_outlet_pos) {
            Some(outlet) => {
                outlet.receivers.push((receiver_id, receiver_inlet_pos));
                Ok(())
            },

            None => Err(GraphError::OutletNotFound {
                id: sender_id,
                outlet_pos: sender_outlet_pos
            })
        }
    }

    pub fn disconnect_nodes(
        &mut self,
        sender_id: u64,
        sender_outlet_pos: usize,
        receiver_id: u64,
        receiver_inlet_pos: usize
    ) -> Result<(), GraphError> {
        let sender = {
            let (table_index, record_index) =
                self.get_index(sender_id).ok_or_else(
                    || GraphError::NodeNotFound {id: sender_id}
                )?;

            &mut self.node_table[table_index][record_index]
        };

        match (*sender.outlets).get_mut(sender_outlet_pos) {
            Some(outlet) => {
                match outlet.receivers.iter().position(
                    |(receiver_id_2, receiver_inlet_pos_2)|
                        (receiver_id == *receiver_id_2)
                            && (receiver_inlet_pos == *receiver_inlet_pos_2)
                ) {
                    Some(position) => {
                        outlet.receivers.remove(position);
                        Ok(())
                    },

                    None =>  Err(GraphError::ReceiverNotFound {
                        id: receiver_id,
                        inlet_pos: receiver_inlet_pos
                    })
                }
            },

            None => Err(GraphError::OutletNotFound {
                id: sender_id,
                outlet_pos: sender_outlet_pos
            })
        }
    }

    pub fn send(
        &mut self,
        id: u64,
        inlet_pos: usize,
        data: Data
    ) -> Result<ChobitFlowResult, GraphError> {
        let node = {
            let (table_index, record_index) = self.get_index(id).ok_or_else(
                || GraphError::NodeNotFound {id: id}
            )?;

            &mut self.node_table[table_index][record_index]
        };

        match node.inlets.get_mut(inlet_pos) {
            Some(inlet) => {
                inlet.sender = None;
                inlet.data = Some(data);

                if inlet_pos == 0 {
                    handle_graph_command!(self, node.receive_hot_inlet()?)
                } else {
                    Ok(ChobitFlowResult::Ended)
                }
            },

            None => Err(GraphError::InletNotFound {
                id: node.id,
                inlet_pos: inlet_pos
            })
        }
    }

    pub fn resume(&mut self) -> Result<ChobitFlowResult, GraphError> {
        let id = self.standby_node_id.take().ok_or_else(
            || GraphError::NoStandbyNode
        )?;

        let node = {
            let (table_index, record_index) = self.get_index(id).ok_or_else(
                || GraphError::NodeNotFound {id: id}
            )?;

            &mut self.node_table[table_index][record_index]
        };

        handle_graph_command!(self, node.resume()?)
    }

    fn continue_output(
        &mut self,
        id: u64
    ) -> Result<ChobitFlowResult, GraphError> {
        let node = {
            let (table_index, record_index) = self.get_index(id).ok_or_else(
                || GraphError::NodeNotFound {id: id}
            )?;

            &mut self.node_table[table_index][record_index]
        };

        handle_graph_command!(self, node.continue_output()?)
    }
}
