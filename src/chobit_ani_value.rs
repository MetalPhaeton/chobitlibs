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
    InvalidFramesPerSecond
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

    saved_time: f32,
    seconds_per_frame: f32
}

pub const MIN_COLUMNS: usize = 1;
pub const MIN_ROWS: usize = 1;
pub const MIN_FRAMES_PER_SECOND: f32 = f32::EPSILON;

impl ChobitAniValue {
    /// - `columns` : Columns of UV frame. (must be 1 or more)
    /// - `frames_of_each_row` : Frames of each row of UV frame. (lenght must be 1 or more)
    pub fn new(
        columns: usize,
        frames_of_each_row: &[usize],
        frames_per_second: f32,
    ) -> Result<Self, ChobitAniValueError> {
        if columns < MIN_COLUMNS {
            return Err(ChobitAniValueError::from(
                GenerationError::InvalidColumns
            ));
        }

        if frames_per_second < MIN_FRAMES_PER_SECOND {
            return Err(ChobitAniValueError::from(
                GenerationError::InvalidFramesPerSecond
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

            saved_time: 0.0,
            seconds_per_frame: frames_per_second.recip()
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
    pub fn current_frame(&self) -> Result<usize, ChobitAniValueError> {
        self.current_frame.ok_or_else(
            || ChobitAniValueError::NoFrameInCurrentRow
        )
    }

    #[inline]
    pub fn current_row(&self) -> usize {self.current_row}

    #[inline]
    pub fn last_frame(&self) -> Result<usize, ChobitAniValueError> {
        debug_assert!(self.last_frame.get(self.current_row).is_some());

        unsafe {
            self.last_frame.get_unchecked(self.current_row).ok_or_else(
                || ChobitAniValueError::NoFrameInCurrentRow
            )
        }
    }

    #[inline]
    pub fn saved_time(&self) -> f32 {self.saved_time}

    #[inline]
    pub fn seconds_per_frame(&self) -> f32 {self.seconds_per_frame}

    #[inline]
    pub fn frames_per_second(&self) -> f32 {self.seconds_per_frame.recip()}

    #[inline]
    pub fn uv_frame_height(&self) -> f32 {self.uv_frame_height}

    #[inline]
    pub fn uv_frame_width(&self) -> f32 {self.uv_frame_width}

    #[inline]
    pub fn uv_frame_left_top_right_bottom(
        &self
    ) -> Result<&(f32, f32, f32, f32), ChobitAniValueError> {
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
        }).ok_or_else(
            || ChobitAniValueError::NoFrameInCurrentRow
        )
    }

    #[inline]
    pub fn set_saved_time(&mut self, time: f32) {
        self.saved_time = time;
    }

    #[inline]
    pub fn set_frames_per_second(&mut self, frames_per_second: f32) {
        self.seconds_per_frame = frames_per_second.recip();
    }

    #[inline]
    pub fn set_frame(
        &mut self,
        frame: usize
    ) -> Result<(), ChobitAniValueError> {
         if frame <= self.last_frame()? {
            self.current_frame = Some(frame);

            Ok(())
        } else {
            Err(ChobitAniValueError::InvalidFrame)
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

    #[inline]
    pub fn next_frame(&mut self) {
        if let Some(frame) = self.current_frame {
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
        }
    }

    #[inline]
    pub fn prev_frame(&mut self) {
        if let Some(frame) = self.current_frame {
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
        }
    }

    #[inline]
    pub fn elapse(&mut self, dt: f32) {
        self.saved_time += dt;

        while self.saved_time >= self.seconds_per_frame {
            self.next_frame();

            self.saved_time -= self.seconds_per_frame;
        }
    }

    #[inline]
    pub fn elapse_inv(&mut self, dt: f32) {
        self.saved_time += dt;

        while self.saved_time >= self.seconds_per_frame {
            self.prev_frame();

            self.saved_time -= self.seconds_per_frame;
        }
    }
}
