// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

/// Helper trait for range
pub trait StepByOne {
    fn step(&mut self);
}

/// A simple range structure
#[derive(Copy, Clone)]
pub struct Range<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd,
{
    l: T,
    r: T,
}
impl<T> Range<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd,
{
    pub fn new(start: T, end: T) -> Self {
        Self { l: start, r: end }
    }
    pub fn get_start(&self) -> T {
        self.l
    }
    pub fn get_end(&self) -> T {
        self.r
    }
}
impl<T> IntoIterator for Range<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd,
{
    type Item = T;
    type IntoIter = RangeIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        RangeIter::new(self.l, self.r)
    }
}

/// Iterator for range
pub struct RangeIter<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd,
{
    current: T,
    end: T,
}
impl<T> RangeIter<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd,
{
    pub fn new(l: T, r: T) -> Self {
        Self { current: l, end: r }
    }
}
impl<T> Iterator for RangeIter<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            None
        } else {
            let t = self.current;
            self.current.step();
            Some(t)
        }
    }
}
