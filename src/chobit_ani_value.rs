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
}

#[derive(Debug, Clone, PartialEq)]
pub enum GenerationError {
    InvalidColumns,
    InvalidFramesOfEachRow,
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
    last_frame: Box<[usize]>,
    last_row: usize,
    next_frame: Box<[Box<[usize]>]>,
    prev_frame: Box<[Box<[usize]>]>,

    current_frame: usize,
    current_row: usize,

    uv_frame_width: f32,
    uv_frame_height: f32,
    uv_frame: Box<[Box<[(f32, f32, f32, f32)]>]>,

    saved_time: f32,
    seconds_per_frame: f32
}

pub const MIN_COLUMNS: usize = 1;
pub const MIN_FRAMES: usize = 1;
pub const MIN_ROWS: usize = 1;
pub const MIN_FRAMES_PER_SECOND: f32 = f32::EPSILON;

impl ChobitAniValue {
    /// - `columns` : Columns of UV frame. (must be 1 or more)
    /// - `frames_of_each_row` : Frames of each row of UV frame. (lenght must be 1 or more and each element must be 1 or more)
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

        let rows = frames_of_each_row.len();

        if rows < MIN_ROWS {
            return Err(ChobitAniValueError::from(
                GenerationError::InvalidRows
            ));
        }

        let mut last_frame = Vec::<usize>::with_capacity(rows);

        for &frames in frames_of_each_row {
            if (frames >= MIN_FRAMES) && (frames <= columns) {
                last_frame.push(frames - 1);
            } else {
                return Err(ChobitAniValueError::from(
                    GenerationError::InvalidFramesOfEachRow
                ));
            }
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

        Ok(Self {
            columns: columns,
            rows: rows,
            last_frame: last_frame.into_boxed_slice(),
            last_row: rows - 1,
            next_frame: next_frame,
            prev_frame: prev_frame,

            current_frame: 0,
            current_row: 0,

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
        last_frame: &[usize]
    ) -> Box<[Box<[usize]>]> {
        (0..columns).map(move |col| {
            (0..rows).map(move |ro| {
                let last_frame = last_frame[ro];

                if col == last_frame {
                    0
                } else {
                    col + 1
                }
            }).collect::<Vec<usize>>().into_boxed_slice()
        }).collect::<Vec<Box<[usize]>>>().into_boxed_slice()
    }

    fn gen_prev_frame(
        columns: usize,
        rows: usize,
        last_frame: &[usize]
    ) -> Box<[Box<[usize]>]> {
        (0..columns).map(move |col| {
            (0..rows).map(move |ro| {
                let last_frame = last_frame[ro];

                if col == 0 {
                    last_frame
                } else {
                    col - 1
                }
            }).collect::<Vec<usize>>().into_boxed_slice()
        }).collect::<Vec<Box<[usize]>>>().into_boxed_slice()
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
    pub fn current_frame(&self) -> usize {self.current_frame}

    #[inline]
    pub fn current_row(&self) -> usize {self.current_row}

    #[inline]
    pub fn last_frame(&self) -> usize {
        debug_assert!(self.last_frame.get(self.current_row).is_some());

        unsafe {*self.last_frame.get_unchecked(self.current_row)}
    }

    #[inline]
    pub fn saved_time(&self) -> f32 {self.saved_time}

    #[inline]
    pub fn saved_time_mut(&mut self) -> &mut f32 {&mut self.saved_time}

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
    ) -> &(f32, f32, f32, f32) {
        debug_assert!(self.uv_frame.get(self.current_frame).is_some());

        unsafe {
            debug_assert!(self.uv_frame.get_unchecked(
                self.current_frame
            ).get(
                self.current_row
            ).is_some());

            self.uv_frame.get_unchecked(
                self.current_frame
            ).get_unchecked(
                self.current_row
            )
        }
    }

    #[inline]
    pub fn set_frames_per_second(&mut self, frames_per_second: f32) {
        self.seconds_per_frame = frames_per_second.recip();
    }

    #[inline]
    pub fn set_frame(&mut self, frame: usize) {
        self.saved_time = 0.0;
        self.current_frame = self.last_frame().min(frame);
    }

    #[inline]
    pub fn set_row(&mut self, row: usize) {
        self.saved_time = 0.0;
        self.current_row = self.rows.min(row);
        self.current_frame = 0;
    }

    #[inline]
    fn next_frame_core(&mut self) -> usize {
        debug_assert!(self.next_frame.get(self.current_frame).is_some());

        self.current_frame = unsafe {
            debug_assert!(self.next_frame.get_unchecked(
                self.current_frame
            ).get(
                self.current_row
            ).is_some());

            *self.next_frame.get_unchecked(
                self.current_frame
            ).get_unchecked(
                self.current_row
            )
        };

        self.current_frame
    }

    #[inline]
    pub fn next_frame(&mut self) -> usize {
        self.saved_time = 0.0;
        self.next_frame_core()
    }

    #[inline]
    fn prev_frame_core(&mut self) -> usize {
        debug_assert!(self.next_frame.get(self.current_frame).is_some());

        self.current_frame = unsafe {
            debug_assert!(self.prev_frame.get_unchecked(
                self.current_frame
            ).get(
                self.current_row
            ).is_some());

            *self.prev_frame.get_unchecked(
                self.current_frame
            ).get_unchecked(
                self.current_row
            )
        };

        self.current_frame
    }

    #[inline]
    pub fn prev_frame(&mut self) -> usize {
        self.saved_time = 0.0;
        self.prev_frame_core()
    }

    #[inline]
    pub fn elapse(&mut self, dt: f32) -> usize {
        self.saved_time += dt;

        let mut ret: usize = self.current_frame;

        while self.saved_time >= self.seconds_per_frame {
            ret = self.next_frame_core();

            self.saved_time -= self.seconds_per_frame;
        }

        ret
    }

    #[inline]
    pub fn elapse_inv(&mut self, dt: f32) -> usize {
        self.saved_time += dt;

        let mut ret: usize = self.current_frame;

        while self.saved_time >= self.seconds_per_frame {
            ret = self.prev_frame_core();

            self.saved_time -= self.seconds_per_frame;
        }

        ret
    }
}
