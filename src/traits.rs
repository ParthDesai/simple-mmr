use std::ops::{Add, Index, IndexMut, Sub};
use std::slice::SliceIndex;

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

pub trait SimpleMMRStorage<Id, O>: Index<Id, Output = O> + IndexMut<Id, Output = O> {
    fn length(&self) -> usize;
    fn capacity(&self) -> usize;
    fn increase_capacity(&mut self, increase_by: usize) -> usize;
    fn decrease_capacity(&mut self, decrease_by: usize) -> usize;
}

impl<H, Err, Id, O> SimpleMMR<H, Err> for dyn SimpleMMRStorage<Id, O>
where
    Id: Eq + Ord + Add + Sub,
    H: Hashable<Output = O>,
    Err: std::error::Error,
{
    fn append(&mut self, data: H) -> Result<(), Err> {
        let hash = data.hash();
        let current_length = self.length();
        if current_length == self.capacity() {
            self.increase_capacity(1);
        }
        todo!()
    }

    fn prune(&mut self) -> Result<(), Err> {
        todo!()
    }

    fn root(&self) -> H::Output {
        todo!()
    }
}
