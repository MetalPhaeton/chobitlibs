// Copyright (C) 2024 Hironori Ishibashi
//
// This work is free. You can redistribute it and/or modify it under the
// terms of the Do What The Fuck You Want To Public License, Version 2,
// as published by Sam Hocevar. See below for more details.
//
// --------------------------------------------------------------------
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

//! Flowchart framework.
//!
//! # Example
//!
//! <svg aria-roledescription="flowchart-v2" role="graphics-document document" viewBox="-8 -8 246.4375 233" style="max-width: 246.438px; background-color: white;" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns="http://www.w3.org/2000/svg" width="100%" id="my-svg"><style>#my-svg{font-family:"trebuchet ms",verdana,arial,sans-serif;font-size:16px;fill:#333;}#my-svg .error-icon{fill:#552222;}#my-svg .error-text{fill:#552222;stroke:#552222;}#my-svg .edge-thickness-normal{stroke-width:2px;}#my-svg .edge-thickness-thick{stroke-width:3.5px;}#my-svg .edge-pattern-solid{stroke-dasharray:0;}#my-svg .edge-pattern-dashed{stroke-dasharray:3;}#my-svg .edge-pattern-dotted{stroke-dasharray:2;}#my-svg .marker{fill:#333333;stroke:#333333;}#my-svg .marker.cross{stroke:#333333;}#my-svg svg{font-family:"trebuchet ms",verdana,arial,sans-serif;font-size:16px;}#my-svg .label{font-family:"trebuchet ms",verdana,arial,sans-serif;color:#333;}#my-svg .cluster-label text{fill:#333;}#my-svg .cluster-label span,#my-svg p{color:#333;}#my-svg .label text,#my-svg span,#my-svg p{fill:#333;color:#333;}#my-svg .node rect,#my-svg .node circle,#my-svg .node ellipse,#my-svg .node polygon,#my-svg .node path{fill:#ECECFF;stroke:#9370DB;stroke-width:1px;}#my-svg .flowchart-label text{text-anchor:middle;}#my-svg .node .katex path{fill:#000;stroke:#000;stroke-width:1px;}#my-svg .node .label{text-align:center;}#my-svg .node.clickable{cursor:pointer;}#my-svg .arrowheadPath{fill:#333333;}#my-svg .edgePath .path{stroke:#333333;stroke-width:2.0px;}#my-svg .flowchart-link{stroke:#333333;fill:none;}#my-svg .edgeLabel{background-color:#e8e8e8;text-align:center;}#my-svg .edgeLabel rect{opacity:0.5;background-color:#e8e8e8;fill:#e8e8e8;}#my-svg .labelBkg{background-color:rgba(232, 232, 232, 0.5);}#my-svg .cluster rect{fill:#ffffde;stroke:#aaaa33;stroke-width:1px;}#my-svg .cluster text{fill:#333;}#my-svg .cluster span,#my-svg p{color:#333;}#my-svg div.mermaidTooltip{position:absolute;text-align:center;max-width:200px;padding:2px;font-family:"trebuchet ms",verdana,arial,sans-serif;font-size:12px;background:hsl(80, 100%, 96.2745098039%);border:1px solid #aaaa33;border-radius:2px;pointer-events:none;z-index:100;}#my-svg .flowchartTitleText{text-anchor:middle;font-size:18px;fill:#333;}#my-svg :root{--mermaid-font-family:"trebuchet ms",verdana,arial,sans-serif;}</style><g><marker orient="auto" markerHeight="12" markerWidth="12" markerUnits="userSpaceOnUse" refY="5" refX="6" viewBox="0 0 10 10" class="marker flowchart" id="my-svg_flowchart-pointEnd"><path style="stroke-width: 1; stroke-dasharray: 1, 0;" class="arrowMarkerPath" d="M 0 0 L 10 5 L 0 10 z"/></marker><marker orient="auto" markerHeight="12" markerWidth="12" markerUnits="userSpaceOnUse" refY="5" refX="4.5" viewBox="0 0 10 10" class="marker flowchart" id="my-svg_flowchart-pointStart"><path style="stroke-width: 1; stroke-dasharray: 1, 0;" class="arrowMarkerPath" d="M 0 5 L 10 10 L 10 0 z"/></marker><marker orient="auto" markerHeight="11" markerWidth="11" markerUnits="userSpaceOnUse" refY="5" refX="11" viewBox="0 0 10 10" class="marker flowchart" id="my-svg_flowchart-circleEnd"><circle style="stroke-width: 1; stroke-dasharray: 1, 0;" class="arrowMarkerPath" r="5" cy="5" cx="5"/></marker><marker orient="auto" markerHeight="11" markerWidth="11" markerUnits="userSpaceOnUse" refY="5" refX="-1" viewBox="0 0 10 10" class="marker flowchart" id="my-svg_flowchart-circleStart"><circle style="stroke-width: 1; stroke-dasharray: 1, 0;" class="arrowMarkerPath" r="5" cy="5" cx="5"/></marker><marker orient="auto" markerHeight="11" markerWidth="11" markerUnits="userSpaceOnUse" refY="5.2" refX="12" viewBox="0 0 11 11" class="marker cross flowchart" id="my-svg_flowchart-crossEnd"><path style="stroke-width: 2; stroke-dasharray: 1, 0;" class="arrowMarkerPath" d="M 1,1 l 9,9 M 10,1 l -9,9"/></marker><marker orient="auto" markerHeight="11" markerWidth="11" markerUnits="userSpaceOnUse" refY="5.2" refX="-1" viewBox="0 0 11 11" class="marker cross flowchart" id="my-svg_flowchart-crossStart"><path style="stroke-width: 2; stroke-dasharray: 1, 0;" class="arrowMarkerPath" d="M 1,1 l 9,9 M 10,1 l -9,9"/></marker><g class="root"><g class="clusters"/><g class="edgePaths"><path marker-end="url(#my-svg_flowchart-pointEnd)" style="fill:none;" class="edge-thickness-normal edge-pattern-solid flowchart-link LS-a LE-b" id="L-a-b-0" d="M90.452,33L82.862,38.667C75.272,44.333,60.093,55.667,52.504,66.117C44.914,76.567,44.914,86.133,44.914,90.917L44.914,95.7"/><path marker-end="url(#my-svg_flowchart-pointEnd)" style="fill:none;" class="edge-thickness-normal edge-pattern-solid flowchart-link LS-a LE-c" id="L-a-c-0" d="M134.65,33L142.24,38.667C149.829,44.333,165.008,55.667,172.598,66.117C180.188,76.567,180.188,86.133,180.188,90.917L180.188,95.7"/><path marker-end="url(#my-svg_flowchart-pointEnd)" style="fill:none;" class="edge-thickness-normal edge-pattern-solid flowchart-link LS-b LE-d" id="L-b-d-0" d="M44.914,134L44.914,138.167C44.914,142.333,44.914,150.667,50.952,158.538C56.99,166.409,69.066,173.819,75.104,177.524L81.142,181.228"/><path marker-end="url(#my-svg_flowchart-pointEnd)" style="fill:none;" class="edge-thickness-normal edge-pattern-solid flowchart-link LS-c LE-d" id="L-c-d-0" d="M180.188,134L180.188,138.167C180.188,142.333,180.188,150.667,174.15,158.538C168.112,166.409,156.036,173.819,149.998,177.524L143.96,181.228"/></g><g class="edgeLabels"><g transform="translate(44.9140625, 67)" class="edgeLabel"><g transform="translate(-44.9140625, -9)" class="label"><foreignObject height="18" width="89.828125"><div style="display: inline-block; white-space: nowrap;" xmlns="http://www.w3.org/1999/xhtml"><span class="edgeLabel">Route is Left</span></div></foreignObject></g></g><g transform="translate(180.1875, 67)" class="edgeLabel"><g transform="translate(-50.25, -9)" class="label"><foreignObject height="18" width="100.5"><div style="display: inline-block; white-space: nowrap;" xmlns="http://www.w3.org/1999/xhtml"><span class="edgeLabel">Route is Right</span></div></foreignObject></g></g><g class="edgeLabel"><g transform="translate(0, 0)" class="label"><foreignObject height="0" width="0"><div style="display: inline-block; white-space: nowrap;" xmlns="http://www.w3.org/1999/xhtml"><span class="edgeLabel"></span></div></foreignObject></g></g><g class="edgeLabel"><g transform="translate(0, 0)" class="label"><foreignObject height="0" width="0"><div style="display: inline-block; white-space: nowrap;" xmlns="http://www.w3.org/1999/xhtml"><span class="edgeLabel"></span></div></foreignObject></g></g></g><g class="nodes"><g transform="translate(112.55078125, 16.5)" data-id="a" data-node="true" id="flowchart-a-0" class="node default default flowchart-label"><rect height="33" width="87.046875" y="-16.5" x="-43.5234375" ry="0" rx="0" style="" class="basic label-container"/><g transform="translate(-36.0234375, -9)" style="" class="label"><rect/><foreignObject height="18" width="72.046875"><div style="display: inline-block; white-space: nowrap;" xmlns="http://www.w3.org/1999/xhtml"><span class="nodeLabel">NodeStart</span></div></foreignObject></g></g><g transform="translate(44.9140625, 117.5)" data-id="b" data-node="true" id="flowchart-b-1" class="node default default flowchart-label"><rect height="33" width="79.9375" y="-16.5" x="-39.96875" ry="0" rx="0" style="" class="basic label-container"/><g transform="translate(-32.46875, -9)" style="" class="label"><rect/><foreignObject height="18" width="64.9375"><div style="display: inline-block; white-space: nowrap;" xmlns="http://www.w3.org/1999/xhtml"><span class="nodeLabel">NodeLeft</span></div></foreignObject></g></g><g transform="translate(180.1875, 117.5)" data-id="c" data-node="true" id="flowchart-c-3" class="node default default flowchart-label"><rect height="33" width="90.609375" y="-16.5" x="-45.3046875" ry="0" rx="0" style="" class="basic label-container"/><g transform="translate(-37.8046875, -9)" style="" class="label"><rect/><foreignObject height="18" width="75.609375"><div style="display: inline-block; white-space: nowrap;" xmlns="http://www.w3.org/1999/xhtml"><span class="nodeLabel">NodeRight</span></div></foreignObject></g></g><g transform="translate(112.55078125, 200.5)" data-id="d" data-node="true" id="flowchart-d-5" class="node default default flowchart-label"><rect height="33" width="81.71875" y="-16.5" x="-40.859375" ry="0" rx="0" style="" class="basic label-container"/><g transform="translate(-33.359375, -9)" style="" class="label"><rect/><foreignObject height="18" width="66.71875"><div style="display: inline-block; white-space: nowrap;" xmlns="http://www.w3.org/1999/xhtml"><span class="nodeLabel">NodeEnd</span></div></foreignObject></g></g></g></g></g></svg>
//!
//! Runs through flowchart.
//!
//! ```ignore
//! use chobitlibs::chobit_flow::{
//!     ChobitFlowError,
//!     Node,
//!     IdToNode,
//!     ChobitFlow
//! };
//!
//! use std::rc::Rc;
//! use std::cell::RefCell;
//!
//! // ID of each node.
//! const ID_START: u64 = 0;
//! const ID_LEFT: u64 = 1;
//! const ID_RIGHT: u64 = 2;
//! const ID_END: u64 = 3;
//!
//! // Message
//! enum Route {
//!     Left,
//!     Right
//! }
//!
//! struct Message {
//!     pub route: Route,
//!     pub log: Vec<u64>
//! }
//!
//! struct NodeStart;
//! impl Node for NodeStart {
//!     type Message = Message;
//!
//!     fn id(&self) -> u64 {ID_START}
//!
//!     fn execute(
//!         &mut self,
//!         prev_id: Option<u64>,
//!         mut message: Message
//!     ) -> (Option<u64>, Self::Message) {
//!         message.log.push(self.id());
//!
//!         match message.route {
//!             Route::Left => (Some(ID_LEFT), message),
//!             Route::Right => (Some(ID_RIGHT), message)
//!         }
//!     }
//! }
//!
//! struct NodeLeft;
//! impl Node for NodeLeft {
//!     type Message = Message;
//!
//!     fn id(&self) -> u64 {ID_LEFT}
//!
//!     fn execute(
//!         &mut self,
//!         prev_id: Option<u64>,
//!         mut message: Message
//!     ) -> (Option<u64>, Self::Message) {
//!         message.log.push(self.id());
//!
//!         (Some(ID_END), message)
//!     }
//! }
//!
//! struct NodeRight;
//! impl Node for NodeRight {
//!     type Message = Message;
//!
//!     fn id(&self) -> u64 {ID_RIGHT}
//!
//!     fn execute(
//!         &mut self,
//!         prev_id: Option<u64>,
//!         mut message: Message
//!     ) -> (Option<u64>, Self::Message) {
//!         message.log.push(self.id());
//!
//!         (Some(ID_END), message)
//!     }
//! }
//!
//! struct NodeEnd;
//! impl Node for NodeEnd {
//!     type Message = Message;
//!
//!     fn id(&self) -> u64 {ID_END}
//!
//!     fn execute(
//!         &mut self,
//!         prev_id: Option<u64>,
//!         mut message: Message
//!     ) -> (Option<u64>, Self::Message) {
//!         message.log.push(self.id());
//!
//!         (None, message)
//!     }
//! }
//!
//! // Converter from ID to Node.
//! struct IdConverter {
//!     node_start: Rc<RefCell<NodeStart>>,
//!     node_left: Rc<RefCell<NodeLeft>>,
//!     node_right: Rc<RefCell<NodeRight>>,
//!     node_end: Rc<RefCell<NodeEnd>>,
//! }
//! impl  IdConverter {
//!     fn new() -> Self {
//!         Self {
//!             node_start: Rc::new(RefCell::new(NodeStart)),
//!             node_left: Rc::new(RefCell::new(NodeLeft)),
//!             node_right: Rc::new(RefCell::new(NodeRight)),
//!             node_end: Rc::new(RefCell::new(NodeEnd))
//!         }
//!     }
//! }
//! impl IdToNode for IdConverter {
//!     type Message = Message;
//!
//!     fn id_to_node(
//!         &mut self,
//!         id: u64
//!     ) -> Option<Rc<RefCell<dyn Node<Message = Message>>>> {
//!         match id {
//!             ID_START => Some(self.node_start.clone()),
//!             ID_LEFT => Some(self.node_left.clone()),
//!             ID_RIGHT => Some(self.node_right.clone()),
//!             ID_END => Some(self.node_end.clone()),
//!             _ => None
//!         }
//!     }
//! }
//!
//! // Create ChobitFlow.
//! let mut graph = ChobitFlow::<Message, IdConverter>::new(
//!     ID_START,
//!     IdConverter::new()
//! );
//!
//! // Route left
//! let message = Message {
//!     route: Route::Left,
//!     log: Vec::<u64>::new()
//! };
//!
//! // Run route left.
//! let message = graph.run(None, message).unwrap();
//!
//! // Check log and last ID.
//! assert_eq!(message.log.as_slice(), &[ID_START, ID_LEFT, ID_END]);
//! assert_eq!(graph.next_id(), ID_END);
//!
//! // Reset start node.
//! graph.set_next_id(ID_START);
//!
//! // Route right
//! let message = Message {
//!     route: Route::Right,
//!     log: Vec::<u64>::new()
//! };
//!
//! // Run route left.
//! let message = graph.run(None, message).unwrap();
//!
//! // Check log and last ID.
//! assert_eq!(message.log.as_slice(), &[ID_START, ID_RIGHT, ID_END]);
//! assert_eq!(graph.next_id(), ID_END);
//! ```
//!
//! Runs step by step.
//!
//! ```ignore
//! use chobitlibs::chobit_flow::{
//!     ChobitFlowError,
//!     Node,
//!     IdToNode,
//!     ChobitFlow
//! };
//!
//! use std::rc::Rc;
//! use std::cell::RefCell;
//!
//! // ID of each node.
//! const ID_START: u64 = 0;
//! const ID_LEFT: u64 = 1;
//! const ID_RIGHT: u64 = 2;
//! const ID_END: u64 = 3;
//!
//! // Message
//! enum Route {
//!     Left,
//!     Right
//! }
//!
//! struct Message {
//!     pub route: Route,
//!     pub log: Vec<u64>
//! }
//!
//! struct NodeStart;
//! impl Node for NodeStart {
//!     type Message = Message;
//!
//!     fn id(&self) -> u64 {ID_START}
//!
//!     fn execute(
//!         &mut self,
//!         prev_id: Option<u64>,
//!         mut message: Message
//!     ) -> (Option<u64>, Self::Message) {
//!         message.log.push(self.id());
//!
//!         match message.route {
//!             Route::Left => (Some(ID_LEFT), message),
//!             Route::Right => (Some(ID_RIGHT), message)
//!         }
//!     }
//! }
//!
//! struct NodeLeft;
//! impl Node for NodeLeft {
//!     type Message = Message;
//!
//!     fn id(&self) -> u64 {ID_LEFT}
//!
//!     fn execute(
//!         &mut self,
//!         prev_id: Option<u64>,
//!         mut message: Message
//!     ) -> (Option<u64>, Self::Message) {
//!         message.log.push(self.id());
//!
//!         (Some(ID_END), message)
//!     }
//! }
//!
//! struct NodeRight;
//! impl Node for NodeRight {
//!     type Message = Message;
//!
//!     fn id(&self) -> u64 {ID_RIGHT}
//!
//!     fn execute(
//!         &mut self,
//!         prev_id: Option<u64>,
//!         mut message: Message
//!     ) -> (Option<u64>, Self::Message) {
//!         message.log.push(self.id());
//!
//!         (Some(ID_END), message)
//!     }
//! }
//!
//! struct NodeEnd;
//! impl Node for NodeEnd {
//!     type Message = Message;
//!
//!     fn id(&self) -> u64 {ID_END}
//!
//!     fn execute(
//!         &mut self,
//!         prev_id: Option<u64>,
//!         mut message: Message
//!     ) -> (Option<u64>, Self::Message) {
//!         message.log.push(self.id());
//!
//!         (None, message)
//!     }
//! }
//!
//! // Converter from ID to Node.
//! struct IdConverter {
//!     node_start: Rc<RefCell<NodeStart>>,
//!     node_left: Rc<RefCell<NodeLeft>>,
//!     node_right: Rc<RefCell<NodeRight>>,
//!     node_end: Rc<RefCell<NodeEnd>>,
//! }
//! impl  IdConverter {
//!     fn new() -> Self {
//!         Self {
//!             node_start: Rc::new(RefCell::new(NodeStart)),
//!             node_left: Rc::new(RefCell::new(NodeLeft)),
//!             node_right: Rc::new(RefCell::new(NodeRight)),
//!             node_end: Rc::new(RefCell::new(NodeEnd))
//!         }
//!     }
//! }
//! impl IdToNode for IdConverter {
//!     type Message = Message;
//!
//!     fn id_to_node(
//!         &mut self,
//!         id: u64
//!     ) -> Option<Rc<RefCell<dyn Node<Message = Message>>>> {
//!         match id {
//!             ID_START => Some(self.node_start.clone()),
//!             ID_LEFT => Some(self.node_left.clone()),
//!             ID_RIGHT => Some(self.node_right.clone()),
//!             ID_END => Some(self.node_end.clone()),
//!             _ => None
//!         }
//!     }
//! }
//!
//! // Create ChobitFlow.
//! let mut graph = ChobitFlow::<Message, IdConverter>::new(
//!     ID_START,
//!     IdConverter::new()
//! );
//!
//! // Route left
//! let message = Message {
//!     route: Route::Left,
//!     log: Vec::<u64>::new()
//! };
//!
//!  // Executes NodeStart.
//! let (opt_next_id, message) = graph.next(None, message).unwrap();
//! assert_eq!(graph.next_id(), ID_LEFT);
//! assert_eq!(opt_next_id, Some(ID_LEFT));
//!
//!  // Executes NodeLeft.
//! let (opt_next_id, message) = graph.next(opt_next_id, message).unwrap();
//! assert_eq!(graph.next_id(), ID_END);
//! assert_eq!(opt_next_id, Some(ID_END));
//!
//!  // Executes NodeEnd.
//! let (opt_next_id, message) = graph.next(opt_next_id, message).unwrap();
//! assert_eq!(graph.next_id(), ID_END);
//! assert_eq!(opt_next_id, None);
//!
//! // Check log and last ID.
//! assert_eq!(message.log.as_slice(), &[ID_START, ID_LEFT, ID_END]);
//! assert_eq!(graph.next_id(), ID_END);
//!
//! // Reset start node.
//! graph.set_next_id(ID_START);
//!
//! // Route right
//! let message = Message {
//!     route: Route::Right,
//!     log: Vec::<u64>::new()
//! };
//!
//!  // Executes NodeStart.
//! let (opt_next_id, message) = graph.next(None, message).unwrap();
//! assert_eq!(graph.next_id(), ID_RIGHT);
//! assert_eq!(opt_next_id, Some(ID_RIGHT));
//!
//!  // Executes NodeRight.
//! let (opt_next_id, message) = graph.next(opt_next_id, message).unwrap();
//! assert_eq!(graph.next_id(), ID_END);
//! assert_eq!(opt_next_id, Some(ID_END));
//!
//!  // Executes NodeEnd.
//! let (opt_next_id, message) = graph.next(opt_next_id, message).unwrap();
//! assert_eq!(graph.next_id(), ID_END);
//! assert_eq!(opt_next_id, None);
//!
//! // Check log and last ID.
//! assert_eq!(message.log.as_slice(), &[ID_START, ID_RIGHT, ID_END]);
//! assert_eq!(graph.next_id(), ID_END);
//! ```

use core::{fmt, cell::RefCell};
use alloc::rc::Rc;

/// Error for [ChobitFlow]
#[derive(Debug, Clone, PartialEq)]
pub enum ChobitFlowError {
    /// ID is not found.
    ///
    /// - `id` : ID.
    NodeNotFound {id: u64},

    /// [ChobitFlow] couldn't borrow node.
    ///
    /// - `id` : ID.
    FailedToBorrowNode {id: u64}
}

impl fmt::Display for ChobitFlowError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, r#"{{"error":"ChobitFlowError","#)?;

        match self {
            Self::NodeNotFound {id} => {
                write!(
                    formatter,
                    r#""kind":"NodeNotFound","id":{}"#,
                    id,
                )?;
            },

            Self::FailedToBorrowNode {id} => {
                write!(
                    formatter,
                    r#""kind":"FailedToBorrowNode","id":{}"#,
                    id,
                )?;
            }
        }

        write!(formatter, "}}")
    }
}

/// A node of flowchart
pub trait Node {
    /// Type of a message.
    type Message;

    /// ID of this node.
    ///
    /// - __Return__ : ID
    fn id(&self) -> u64;

    /// Executes this node
    ///
    /// - `prev_id` : Previous node ID.
    /// - `message` : Message from previous node.
    /// - __Return__ : 1st of tupple is next node ID. (It is None if this node is termination). 2nd of tupple is a message for next node.
    fn execute(
        &mut self,
        prev_id: Option<u64>,
        message: Self::Message
    ) -> (Option<u64>, Self::Message);
}

/// Node Generater from ID.
pub trait IdToNode {
    type Message;

    /// Generates Node from ID.
    /// - `id` : ID.
    /// - __Return__ : Node.
    fn id_to_node(
        &mut self,
        id: u64
    ) -> Option<Rc<RefCell<dyn Node<Message = Self::Message>>>>;
}

/// Flowchart.
///
/// - `M` : Message type.
/// - `F` : Node generator type.
///
pub struct ChobitFlow<M, F: IdToNode<Message = M>> {
    next_id: u64,
    id_to_node: F
}

impl<M, F: IdToNode<Message = M>> ChobitFlow<M, F>{
    /// Creates [ChobitFlow].
    ///
    /// - `initial_node` : Initial node ID.
    /// - `id_to_node` : Node generater.
    /// - __Return__ : [ChobitFlow]
    #[inline]
    pub fn new(initial_node: u64, id_to_node: F) -> Self {
        Self {
            next_id: initial_node,
            id_to_node: id_to_node
        }
    }

    /// Gets next node ID.
    ///
    /// - __Return__ : Next node ID.
    #[inline]
    pub fn next_id(&self) -> u64 {self.next_id}

    /// Sets next node ID.
    ///
    /// - `next_id` : Next node ID.
    #[inline]
    pub fn set_next_id(&mut self, next_id: u64) {self.next_id = next_id;}

    /// Executes next node.  
    /// If the next node returns node ID, next node is set to it.  
    /// If the next node doesn't return node ID, next node doesn't change.
    ///
    /// - `prev_id` : Previous node ID.
    /// - `message` : A message that is sent to next node.
    /// - __Return__ : A message that the next node returns, or an error.
    pub fn next(
        &mut self,
        prev_id: Option<u64>,
        message: M
    ) -> Result<(Option<u64>, M), ChobitFlowError> {
        let node = match self.id_to_node.id_to_node(self.next_id) {
            Some(node) => node,

            None => {
                return Err(ChobitFlowError::NodeNotFound {id: self.next_id});
            }
        };

        let (opt_next_id, next_message) = match node.try_borrow_mut() {
            Ok(mut node) => node.execute(prev_id, message),

            Err(..) => {
                return Err(ChobitFlowError::FailedToBorrowNode {
                    id: self.next_id
                });
            }
        };

        if let Some(next_id) = opt_next_id {
            self.next_id = next_id;
        }

        Ok((opt_next_id, next_message))
    }

    /// Continuous executes nodes until next node returns None.
    ///
    /// - `prev_id` : Previous node ID.
    /// - `message` : A message that is sent to next node.
    /// - __Return__ : A message that a last node returns, or an error.
    pub fn run(
        &mut self,
        mut prev_id: Option<u64>,
        mut message: M
    ) -> Result<M, ChobitFlowError> {
        loop {
            let node = match self.id_to_node.id_to_node(self.next_id) {
                Some(node) => node,

                None => {
                    return Err(
                        ChobitFlowError::NodeNotFound {id: self.next_id}
                    );
                }
            };

            let (opt_next_id, next_message) = match node.try_borrow_mut() {
                Ok(mut node) => node.execute(prev_id, message),

                Err(..) => {
                    return Err(ChobitFlowError::FailedToBorrowNode {
                        id: self.next_id
                    });
                }
            };

            match opt_next_id {
                Some(next_id) => {
                    prev_id = Some(self.next_id);
                    self.next_id = next_id;

                    message = next_message;
                },

                None => {
                    break Ok(next_message);
                }
            }
        }
    }
}
