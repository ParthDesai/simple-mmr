use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

pub fn right_sibling(pos: usize, height: u32) -> usize {
    pos + (2usize.pow(height + 1) - 1)
}

pub fn left_sibling(pos: usize, height: u32) -> usize {
    pos - (2usize.pow(height + 1) - 1)
}

pub fn peak(max_pos: usize) -> usize {
    f64::log2((max_pos + 1) as f64) as usize - 1
}

pub fn left_child(pos: usize, height: u32) -> usize {
    pos - (2usize.pow(height))
}

pub fn right_child(pos: usize) -> usize {
    pos - 1
}

pub trait Hashable {
    type Output;
    fn hash(&self) -> Self::Output;
}

// Id: Eq + Ord + Add + Sub, T: Index<Id> + IndexMut<Id>
pub trait SimpleMMR<Data: Hashable, Error: std::error::Error> {
    fn append(&mut self, data: Data) -> Result<(), Error>;
    fn prune(&mut self) -> Result<(), Error>;
    fn root(&self) -> Data::Output;
}

pub trait SimpleMMRStorage<O>: Index<usize, Output = O> + IndexMut<usize, Output = O> {
    fn length(&self) -> usize;
    fn capacity(&self) -> usize;
    fn increase_capacity(&mut self, increase_by: usize) -> usize;
    fn decrease_capacity(&mut self, decrease_by: usize) -> usize;
}

impl<H, Err, O> SimpleMMR<H, Err> for dyn SimpleMMRStorage<O>
where
    H: Hashable<Output = O>,
    Err: std::error::Error,
{
    fn append(&mut self, data: H) -> Result<(), Err> {
        let hash = data.hash();
        let current_length = self.length();
        todo!()
    }

    fn prune(&mut self) -> Result<(), Err> {
        todo!()
    }

    fn root(&self) -> H::Output {
        todo!()
    }
}
