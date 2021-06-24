use std::ops::{Index, IndexMut};

pub trait Hashable {
    type Output: Default;
    fn hash(&self) -> Self::Output;
}

pub trait Output {
    type Hash;

    fn height(&self) -> u32;
    fn hash(&self) -> Self::Hash;

    fn new(height: u32, hash: Self::Hash) -> Self;
}

pub trait HeightCache {
    fn number_of_elements_at(&self, height: u32) -> usize;
    fn increment_elements_at(&mut self, height: u32);
    fn decrease_element_at(&mut self, height: u32);
}

// Id: Eq + Ord + Add + Sub, T: Index<Id> + IndexMut<Id>
pub trait SimpleMMR<Data: Hashable, Error: std::error::Error> {
    fn append(&mut self, data: Data) -> Result<(), Error>;
    fn root(&self) -> Data::Output;
}

pub trait PrunableMMR<Data: Hashable, Error: std::error::Error>: SimpleMMR<Data, Error> {
    fn prune(&mut self, prune_elements: Vec<usize>) -> Result<(), Error>;
}

pub trait SimpleMMRStorage<O>:
    Index<usize, Output = O> + IndexMut<usize, Output = O> + HeightCache
{
    fn length(&self) -> usize;
    fn capacity(&self) -> usize;
    fn increase_capacity(&mut self, increase_by: usize) -> usize;
}

pub trait SimpleMMRStorageWithDeletion<O>: SimpleMMRStorage<O> {
    fn delete_element(&mut self, elem_index: usize);
}

// Due to Rust's restriction on non-auto trait, we need to create placeholder trait composed of
// PrunableSimpleMMRStorage + SimpleMMR to implement PrunableMMR on all objects that implement
// PrunableSimpleMMRStorage and SimpleMMR
pub trait SimpleMMRForStorageWithDeletion<
    Hash: Default,
    O: Output<Hash = Hash>,
    H: Hashable<Output = Hash>,
    Error: std::error::Error,
>: SimpleMMRStorageWithDeletion<O> + SimpleMMR<H, Error>
{
}
