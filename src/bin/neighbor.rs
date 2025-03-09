
pub struct NeighborIter {
    idx: usize,
    vals: [Option<(usize, usize)>; 4],
}

impl NeighborIter {
    pub fn new(pos: (usize, usize), max_dim: usize) -> Self {
        let mut vals = [None; 4];

        if pos.0 > 0 {
            vals[0] = Some((pos.0 - 1, pos.1));
        }

        if pos.0 < max_dim {
            vals[1] = Some((pos.0 + 1, pos.1));
        }

        if pos.1 > 0 {
            vals[2] = Some((pos.0, pos.1 - 1));
        }

        if pos.1 < max_dim {
            vals[3] = Some((pos.0, pos.1 + 1));
        }

        Self { idx: 0, vals }
    }
}

impl Iterator for NeighborIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < self.vals.len() {
            let val = self.vals[self.idx];
            self.idx += 1;

            if val.is_some() {
                return val;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;
    
    fn assert_equal_unordered<T>(mut a: Vec<T>, mut b: Vec<T>)
    where
        T: Ord + Debug,
    {
        a.sort();
        b.sort();
        assert_eq!(a, b);
    }

    #[test]
    fn test_neighbor_iter() {
        let ni = NeighborIter::new((1, 1), 2);
        let vals: Vec<(usize, usize)> = ni.collect();
        assert_equal_unordered(vals, vec![(0, 1), (1, 0), (2, 1), (1, 2)]);

        let ni = NeighborIter::new((0, 0), 2);
        let vals: Vec<(usize, usize)> = ni.collect();
        assert_equal_unordered(vals, vec![(0, 1), (1, 0)]);
    }
}
