use std::ops::{Index, IndexMut};

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
    fn prune(&mut self) -> Result<(), Error>;
}

pub trait SimpleMMRStorage<O>:
    Index<usize, Output = O> + IndexMut<usize, Output = O> + HeightCache
{
    fn length(&self) -> usize;
    fn capacity(&self) -> usize;
    fn increase_capacity(&mut self, increase_by: usize) -> usize;
}

pub trait PrunableSimpleMMRStorage<O>: SimpleMMRStorage<O> {
    fn prune_element(&mut self, elem_index: usize);
}

impl<H, Err, O, Hash> SimpleMMR<H, Err> for dyn SimpleMMRStorage<O>
where
    Hash: Default,
    O: Output<Hash = Hash>,
    H: Hashable<Output = Hash>,
    Err: std::error::Error,
{
    fn append(&mut self, data: H) -> Result<(), Err> {
        let hash = data.hash();
        let current_length = self.length();
        let last_element = &self[current_length];
        let last_element_height = last_element.height();

        let mut staged_outputs: Vec<O> = vec![];

        assert_ne!(self.number_of_elements_at(last_element_height) % 2, 0);

        staged_outputs.push(O::new(0, hash));
        self.increment_elements_at(0);

        let mut current_height = 0;

        while self.number_of_elements_at(current_height) % 2 == 0 {
            staged_outputs.push(O::new(
                current_height + 1,
                /** TODO: Calculate hash**/
                Hash::default(),
            ));
            self.increment_elements_at(current_height + 1);
            current_height += 1;
        }

        self.increase_capacity(staged_outputs.len());
        for (i, staged_output) in staged_outputs.drain(..).enumerate() {
            self[current_length + 1 + i] = staged_output;
        }

        Ok(())
    }

    fn root(&self) -> H::Output {
        todo!()
    }
}

// Due to Rust's restriction on non-auto trait, we need to create placeholder trait composed of
// PrunableSimpleMMRStorage + SimpleMMR to implement PrunableMMR on all objects that implement
// PrunableSimpleMMRStorage and SimpleMMR
pub trait PrunableStorageWithSimpleMMR<
    Hash: Default,
    O: Output<Hash = Hash>,
    H: Hashable<Output = Hash>,
    Error: std::error::Error,
>: PrunableSimpleMMRStorage<O> + SimpleMMR<H, Error>
{
}

impl<H, Err, O, Hash> PrunableMMR<H, Err> for dyn PrunableStorageWithSimpleMMR<Hash, O, H, Err>
where
    Hash: Default,
    O: Output<Hash = Hash>,
    H: Hashable<Output = Hash>,
    Err: std::error::Error,
{
    fn prune(&mut self) -> Result<(), Err> {
        todo!()
    }
}
