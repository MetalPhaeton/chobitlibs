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

use alloc::{vec, vec::Vec, string::String, boxed::Box};
use core::fmt;

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
    F32(f32),
    F64(f64),
    Bool(bool),
    Char(char),
    Bytes(Vec<u8>),
    String(String),

    Nil,
    Cons(Box<Data>, Box<Data>)
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperatorCommand {
    Go,
    Stop(Option<Data>),
}

pub trait Operator {
    fn receive(
        &mut self,
        inlets: &[Option<Data>],
        outlets: &mut [Option<Data>]
    ) -> Result<OperatorCommand, Box<dyn OperatorError>>;

    fn resume(
        &mut self,
        data: Option<Data>,
        outlets: &mut [Option<Data>]
    ) -> Result<OperatorCommand, Box<dyn OperatorError>>;
}

#[derive(Debug)]
pub enum GraphError {
    Operator(Box<dyn OperatorError>),
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

struct Node {
    operator: Box<dyn Operator>,

    id: u64,

    senders: Box<[Option<u64>]>,
    inlets: Box<[Option<Data>]>,

    receivers: Box<[Vec<(u64, usize)>]>,
    outlets: Box<[Option<Data>]>,

    standby_data: Option<Data>,

    current_outlet: usize,
    current_receiver: usize
}

macro_rules! handle_operator_result {
    ($self:expr, $result:expr) => {
        match $result {
            Ok(OperatorCommand::Go) => $self.continue_output(),

            Ok(OperatorCommand::Stop(data)) => {
                $self.standby_data = data;
                Ok(GraphCommand::Stopped($self.id))
            },

            Err(error) => Err(GraphError::Operator(error))
        }
    };
}

impl Node {
    #[inline]
    fn new(
        operator: Box<dyn Operator>,
        id: u64,
        inlet_size: usize,
        outlet_size: usize
    ) -> Self {
        Node {
            operator: operator,
            id: id,

            senders: vec![None; inlet_size].into_boxed_slice(),
            inlets: vec![None; inlet_size].into_boxed_slice(),

            receivers: vec![
                Vec::<(u64, usize)>::new(); outlet_size
            ].into_boxed_slice(),
            outlets: vec![None; outlet_size].into_boxed_slice(),

            standby_data: None,
            current_outlet: 0,
            current_receiver: 0
        }
    }

    fn receive_hot_inlet(&mut self) -> Result<GraphCommand, GraphError> {
        self.current_outlet = 0;
        self.current_receiver = 0;

        // init outlets.
        (*self.outlets).iter_mut().for_each(|outlet| {*outlet = None;});

        handle_operator_result!(
            self,
            self.operator.receive(&*self.inlets, &mut *self.outlets)
        )
    }

    fn resume(&mut self) -> Result<GraphCommand, GraphError> {
        self.current_outlet = 0;
        self.current_receiver = 0;

        handle_operator_result!(
            self,
            self.operator.resume(self.standby_data.take(), &mut *self.outlets)
        )
    }

    fn continue_output(&mut self) -> Result<GraphCommand, GraphError> {
        match (
            (*self.receivers).get(self.current_outlet),
            (*self.outlets).get(self.current_outlet)
        ) {
            (Some(receivers), Some(outlet)) =>
                match receivers.get(self.current_receiver) {
                    Some((receiver_id, inlet_pos)) => {
                        self.current_receiver += 1;  // set next receiver.

                        match outlet {
                            Some(data) => Ok(GraphCommand::Send(
                                *receiver_id,
                                *inlet_pos,
                                data.clone()
                            )),

                            // ignore the receiver.
                            None => self.continue_output()
                        }
                    },

                    // go to next outlet.
                    None => {
                        self.current_outlet += 1;
                        self.current_receiver = 0;

                        self.continue_output()
                    }
                },

            // all outlet has been sent, so return to sender of hot inlet.
            _ => match (*self.senders).get(0) {
                Some(Some(sender)) => Ok(GraphCommand::Continue(*sender)),

                _ => Ok(GraphCommand::Ended)  // no sender or no hot inlet.
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
    ($self:expr, $command:expr, $node_id:expr) => {
        match $command {
            GraphCommand::Continue(next_id) => $self.continue_output(next_id),

            GraphCommand::Send(next_id, next_inlet_pos, data) =>
                $self.send(Some($node_id), next_id, next_inlet_pos, data),

            GraphCommand::Ended => Ok(ChobitFlowResult::Ended),

            GraphCommand::Stopped(id) => {
                $self.standby_node_id = Some(id);

                Ok(ChobitFlowResult::Stopped(id))
            }
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

            Err(node_index) => {
                id_vec.insert(node_index, id);
                self.node_table[table_index].insert(
                    node_index,
                    Node::new(operator, id, inlet_size, outlet_size)
                );

                Ok(id)
            }
        }
    }

    pub fn remove_node(&mut self, id: u64) -> Result<(), GraphError> {
        {
            let table_index = (id & self.id_mask) as usize;

            let id_vec = &mut self.id_table[table_index];

            match id_vec.binary_search(&id) {
                Ok(node_index) => {
                    id_vec.remove(node_index);
                },

                Err(..) => {return Err(GraphError::NodeNotFound {id: id});}
            }
        }

        // remove id from all outlets.
        self.node_table.iter_mut().for_each(|node_vec| {
            node_vec.iter_mut().for_each(|node| {
                node.receivers.iter_mut().for_each(|receiver_vec| {
                    loop {
                        match receiver_vec.iter().position(
                            |(receiver_id, _)| *receiver_id == id
                        ) {
                            Some(position) => {
                                receiver_vec.remove(position);
                            },

                            None => {break;}
                        }
                    }
                })
            })
        });

        Ok(())
    }

    #[inline]
    fn get_index(&self, id: u64) -> Option<(usize, usize)> {
        let table_index = (id & self.id_mask) as usize;

        match self.id_table[table_index].binary_search(&id) {
            Ok(node_index) => Some((table_index, node_index)),

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
            let (table_index, node_index) =
                self.get_index(sender_id).ok_or_else(
                    || GraphError::NodeNotFound {id: sender_id}
                )?;

            &mut self.node_table[table_index][node_index]
        };


        match (*sender.receivers).get_mut(sender_outlet_pos) {
            Some(receivers_vec) => {
                receivers_vec.push((receiver_id, receiver_inlet_pos));
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
            let (table_index, node_index) =
                self.get_index(sender_id).ok_or_else(
                    || GraphError::NodeNotFound {id: sender_id}
                )?;

            &mut self.node_table[table_index][node_index]
        };

        match (*sender.receivers).get_mut(sender_outlet_pos) {
            Some(receivers_vec) => {
                match receivers_vec.iter().position(
                    |(receiver_id_2, receiver_inlet_pos_2)|
                        (receiver_id == *receiver_id_2)
                            && (receiver_inlet_pos == *receiver_inlet_pos_2)
                ) {
                    Some(position) => {
                        receivers_vec.remove(position);
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

    pub fn clear_inlet(&mut self, id: u64) -> Result<(), GraphError> {
        let node = {
            let (table_index, node_index) =
                self.get_index(id).ok_or_else(
                    || GraphError::NodeNotFound {id: id}
                )?;

            &mut self.node_table[table_index][node_index]
        };

        node.senders.iter_mut().for_each(|sender| {*sender = None;});
        node.inlets.iter_mut().for_each(|inlet| {*inlet = None;});

        Ok(())
    }

    pub fn send(
        &mut self,
        sender_id: Option<u64>,
        receiver_id: u64,
        inlet_pos: usize,
        data: Data
    ) -> Result<ChobitFlowResult, GraphError> {
        let receiver = {
            let (table_index, node_index) =
                self.get_index(receiver_id).ok_or_else(
                    || GraphError::NodeNotFound {id: receiver_id}
                )?;

            &mut self.node_table[table_index][node_index]
        };

        match (
            receiver.senders.get_mut(inlet_pos),
            receiver.inlets.get_mut(inlet_pos)
        ) {
            (Some(sender), Some(inlet)) => {
                *sender = sender_id;
                *inlet = Some(data);

                if inlet_pos == 0 {
                    handle_graph_command!(
                        self,
                        receiver.receive_hot_inlet()?,
                        receiver_id
                    )
                } else {
                    Ok(ChobitFlowResult::Ended)
                }
            },

            _ => Err(GraphError::InletNotFound {
                id: receiver.id,
                inlet_pos: inlet_pos
            })
        }
    }

    pub fn resume(&mut self) -> Result<ChobitFlowResult, GraphError> {
        let id = self.standby_node_id.take().ok_or_else(
            || GraphError::NoStandbyNode
        )?;

        let node = {
            let (table_index, node_index) = self.get_index(id).ok_or_else(
                || GraphError::NodeNotFound {id: id}
            )?;

            &mut self.node_table[table_index][node_index]
        };

        handle_graph_command!(self, node.resume()?, id)
    }

    fn continue_output(
        &mut self,
        id: u64
    ) -> Result<ChobitFlowResult, GraphError> {
        let node = {
            let (table_index, node_index) = self.get_index(id).ok_or_else(
                || GraphError::NodeNotFound {id: id}
            )?;

            &mut self.node_table[table_index][node_index]
        };

        handle_graph_command!(self, node.continue_output()?, id)
    }
}
