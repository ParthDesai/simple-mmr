use crate::traits::{
    Hashable, Output, PrunableMMR, SimpleMMR, SimpleMMRForStorageWithDeletion, SimpleMMRStorage,
};

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

impl<H, Err, O, Hash> PrunableMMR<H, Err> for dyn SimpleMMRForStorageWithDeletion<Hash, O, H, Err>
where
    Hash: Default,
    O: Output<Hash = Hash>,
    H: Hashable<Output = Hash>,
    Err: std::error::Error,
{
    fn prune(&mut self, prune_elements: Vec<usize>) -> Result<(), Err> {
        for element in prune_elements {
            if self.length() < element {
                // Return an error
            }

            if self[element].height() != 0 {
                // Return an error
            }
        }

        // If left and right siblings both are going to be deleted then delete the parent as well
        // repeat above process till no more childless parent is there.
        // After this, recalculate hash of the parent

        Ok(())
    }
}
