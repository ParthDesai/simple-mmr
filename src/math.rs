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
