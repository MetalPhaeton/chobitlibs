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

//#![allow(dead_code)]

use alloc::{vec::Vec, boxed::Box};

#[derive(Debug, Clone, PartialEq)]
pub enum ChobitAniValueError {
    GenerationError(GenerationError),
    NoFrameInCurrentRow,
    InvalidFrame,
    InvalidRow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GenerationError {
    InvalidColumns,
    InvalidRows,
    InvalidTicksPerSecond,
    InvalidTicksPerFrame,
}

impl From<GenerationError> for ChobitAniValueError {
    #[inline]
    fn from(error: GenerationError) -> Self {
        ChobitAniValueError::GenerationError(error)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChobitAniValue {
    columns: usize,
    rows: usize,
    last_frame: Box<[Option<usize>]>,
    last_row: usize,
    next_frame: Box<[Box<[Option<usize>]>]>,
    prev_frame: Box<[Box<[Option<usize>]>]>,

    current_frame: Option<usize>,
    current_row: usize,

    uv_frame_width: f32,
    uv_frame_height: f32,
    uv_frame: Box<[Box<[(f32, f32, f32, f32)]>]>,

    ticks_per_second: usize,
    accumulated_ticks: usize,
    ticks_per_frame: usize
}

pub const MIN_COLUMNS: usize = 1;
pub const MIN_ROWS: usize = 1;
pub const MIN_TICKS_PER_SECOND: usize = 1;
pub const MIN_TICKS_PER_FRAME: usize = 1;

impl ChobitAniValue {
    /// - `columns` : Columns of UV frame. (must be 1 or more)
    /// - `frames_of_each_row` : Frames of each row of UV frame. (lenght must be 1 or more)
    /// - `ticks_per_second` : Defines ticks per second. (must be 1 or more)
    /// - `ticks_per_frame` : Defines ticks per frame. (must be 1 or more)
    pub fn new(
        columns: usize,
        frames_of_each_row: &[usize],
        ticks_per_second: usize,
        ticks_per_frame: usize
    ) -> Result<Self, ChobitAniValueError> {
        if columns < MIN_COLUMNS {
            return Err(ChobitAniValueError::from(
                GenerationError::InvalidColumns
            ));
        }

        if ticks_per_second < MIN_TICKS_PER_SECOND {
            return Err(ChobitAniValueError::from(
                GenerationError::InvalidTicksPerSecond
            ));
        }

        if ticks_per_frame < MIN_TICKS_PER_FRAME {
            return Err(ChobitAniValueError::from(
                GenerationError::InvalidTicksPerFrame
            ));
        }

        let last_frame = frames_of_each_row.iter().map(
            |&frames| (frames >= 1).then(|| frames.min(columns) - 1)
        ).collect::<Vec<Option<usize>>>().into_boxed_slice();

        let rows = last_frame.len();

        if rows < MIN_ROWS {
            return Err(ChobitAniValueError::from(
                GenerationError::InvalidRows
            ));
        }

        let next_frame = Self::gen_next_frame(columns, rows, &*last_frame);
        let prev_frame = Self::gen_prev_frame(columns, rows, &*last_frame);

        let uv_frame_width = (columns as f32).recip();
        let uv_frame_height = (rows as f32).recip();

        let uv_frame = Self::gen_uv_frame(
            columns,
            rows,
            uv_frame_width,
            uv_frame_height
        );

        let current_row: usize = 0;
        let current_frame: Option<usize> = last_frame[current_row].map(|_| 0);

        Ok(Self {
            columns: columns,
            rows: rows,
            last_frame: last_frame,
            last_row: rows - 1,
            next_frame: next_frame,
            prev_frame: prev_frame,

            current_frame: current_frame,
            current_row: current_row,

            uv_frame_width: uv_frame_width,
            uv_frame_height: uv_frame_height,
            uv_frame: uv_frame,

            ticks_per_second: ticks_per_second,
            accumulated_ticks: 0,
            ticks_per_frame: ticks_per_frame
        })
    }

    fn gen_next_frame(
        columns: usize,
        rows: usize,
        last_frame: &[Option<usize>]
    ) -> Box<[Box<[Option<usize>]>]> {
        (0..columns).map(move |col| {
            (0..rows).map(move |ro| {
                let last_frame = last_frame[ro]?;

                if col == last_frame {
                    Some(0)
                } else if col > last_frame {
                    None
                } else {
                    Some(col + 1)
                }
            }).collect::<Vec<Option<usize>>>().into_boxed_slice()
        }).collect::<Vec<Box<[Option<usize>]>>>().into_boxed_slice()
    }

    fn gen_prev_frame(
        columns: usize,
        rows: usize,
        last_frame: &[Option<usize>]
    ) -> Box<[Box<[Option<usize>]>]> {
        (0..columns).map(move |col| {
            (0..rows).map(move |ro| {
                let last_frame = last_frame[ro]?;

                if col == 0 {
                    Some(last_frame)
                } else if col > last_frame {
                    None
                } else {
                    Some(col - 1)
                }
            }).collect::<Vec<Option<usize>>>().into_boxed_slice()
        }).collect::<Vec<Box<[Option<usize>]>>>().into_boxed_slice()
    }

    fn gen_uv_frame(
        columns: usize,
        rows: usize,
        uv_frame_width: f32,
        uv_frame_height: f32
    ) -> Box<[Box<[(f32, f32, f32, f32)]>]> {
        (0..columns).map(move |col| {
            (0..rows).map(move |ro| {
                let left = (col as f32) * uv_frame_width;
                let top = (ro as f32) * uv_frame_height;
                let right = left + uv_frame_width;
                let bottom = top + uv_frame_height;

                (left, top, right, bottom)
            }).collect::<Vec<(f32, f32, f32, f32)>>().into_boxed_slice()
        }).collect::<Vec<Box<[(f32, f32, f32, f32)]>>>().into_boxed_slice()
    }

    #[inline]
    pub fn columns(&self) -> usize {self.columns}

    #[inline]
    pub fn rows(&self) -> usize {self.rows}

    #[inline]
    pub fn current_frame(&self) -> Option<usize> {self.current_frame}

    #[inline]
    pub fn current_row(&self) -> usize {self.current_row}

    #[inline]
    pub fn last_frame(&self) -> Option<usize> {
        debug_assert!(self.last_frame.get(self.current_row).is_some());

        unsafe {*self.last_frame.get_unchecked(self.current_row)}
    }

    #[inline]
    pub fn ticks_per_second(&self) -> usize {self.ticks_per_second}

    #[inline]
    pub fn ticks_per_frame(&self) -> usize {self.ticks_per_frame}

    #[inline]
    pub fn uv_frame_height(&self) -> f32 {self.uv_frame_height}

    #[inline]
    pub fn uv_frame_width(&self) -> f32 {self.uv_frame_width}

    #[inline]
    pub fn uv_frame_left_top_right_bottom(
        &self
    ) -> Option<&(f32, f32, f32, f32)> {
        self.current_frame.map(|frame| unsafe {
            debug_assert!(self.uv_frame.get(frame).is_some());
            debug_assert!(self.uv_frame.get_unchecked(
                frame
            ).get(
                self.current_row
            ).is_some());

            self.uv_frame.get_unchecked(
                frame
            ).get_unchecked(
                self.current_row
            )
        })
    }

    #[inline]
    pub fn set_frame(
        &mut self,
        frame: usize
    ) -> Result<(), ChobitAniValueError> {
        match self.last_frame() {
            Some(last_frame) => if frame <= last_frame {
                self.current_frame = Some(frame);

                Ok(())
            } else {
                Err(ChobitAniValueError::InvalidFrame)
            },

            None => Err(ChobitAniValueError::NoFrameInCurrentRow)
        }
    }

    #[inline]
    pub fn set_row(
        &mut self,
        row: usize
    ) -> Result<(), ChobitAniValueError> {
        if row >= self.rows {
            return Err(ChobitAniValueError::InvalidRow);
        }

        self.current_row = row;
        self.current_frame = self.last_frame[self.current_row].map(|_| 0);

        Ok(())
    }

    /// - _Return_ : Whether the current column is rewound or not.
    #[inline]
    pub fn next_frame(&mut self) -> bool {
        match self.current_frame {
            Some(frame) => {
                debug_assert!(self.next_frame.get(frame).is_some());

                self.current_frame = unsafe {
                    debug_assert!(self.next_frame.get_unchecked(
                        frame
                    ).get(
                        self.current_row
                    ).is_some());

                    *self.next_frame.get_unchecked(
                        frame
                    ).get_unchecked(
                        self.current_row
                    )
                };

                debug_assert!(self.current_frame.is_some());

                unsafe {self.current_frame.unwrap_unchecked() == 0}
            },

            None => false
        }
    }

    /// - _Return_ : Whether the current column is rewound or not.
    #[inline]
    pub fn prev_frame(&mut self) -> bool {
        match self.current_frame {
            Some(frame) => {
                let ret = frame == 0;

                debug_assert!(self.next_frame.get(frame).is_some());

                self.current_frame = unsafe {
                    debug_assert!(self.prev_frame.get_unchecked(
                        frame
                    ).get(
                        self.current_row
                    ).is_some());

                    *self.prev_frame.get_unchecked(
                        frame
                    ).get_unchecked(
                        self.current_row
                    )
                };

                ret
            },

            None => false
        }
    }

    #[inline]
    pub fn seconds_to_ticks(&self, seconds: f32) -> usize {
        let seconds = seconds.max(0.0);

        (((self.ticks_per_second as f32) * seconds) + 0.5) as usize
    }

    #[inline]
    pub fn ticks_to_seconds(&self, ticks: usize) -> f32 {
        (ticks as f32) / (self.ticks_per_second as f32)
    }

    #[inline]
    pub fn fps_to_tpf(&self, fps: f32) -> usize {
        let fps = fps.max(f32::EPSILON);

        (((self.ticks_per_second as f32) / fps) + 0.5) as usize
    }

    #[inline]
    pub fn tpf_to_fps(&self, tps: usize) -> f32 {
        let tps = (tps as f32).max(f32::EPSILON);

        (self.ticks_per_second as f32) / (tps as f32)
    }

    /// - _Return_ : Whether the current column is rewound or not.
    pub fn elapse_ticks(&mut self, ticks: usize) -> bool {
        self.accumulated_ticks += ticks;

        let mut ret: bool = false;

        while self.accumulated_ticks >= self.ticks_per_frame {
            ret = self.next_frame() || ret;

            self.accumulated_ticks -= self.ticks_per_frame;
        }

        ret
    }

    /// - _Return_ : Whether the current column is rewound or not.
    pub fn elapse_ticks_inv(&mut self, ticks: usize) -> bool {
        self.accumulated_ticks += ticks;

        let mut ret: bool = false;

        while self.accumulated_ticks >= self.ticks_per_frame {
            ret = self.prev_frame() || ret;

            self.accumulated_ticks -= self.ticks_per_frame;
        }

        ret
    }

    /// - _Return_ : Whether the current column is rewound or not.
    #[inline]
    pub fn elapse_seconds(&mut self, seconds: f32) -> bool {
        self.elapse_ticks(self.seconds_to_ticks(seconds))
    }

    /// - _Return_ : Whether the current column is rewound or not.
    #[inline]
    pub fn elapse_seconds_inv(&mut self, seconds: f32) -> bool {
        self.elapse_ticks_inv(self.seconds_to_ticks(seconds))
    }
}
