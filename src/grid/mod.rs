enum Existence {
    Positive,
    Negative,
    Nonexistent,
}

trait NegativeIndexed<U: Default> {
    fn existence(&self, index: isize) -> Existence {
        if index >= 0 && (index as usize) < Self::positive_len(self) {
            Existence::Positive
        } else if index < 0 && (index.abs() as usize) <= Self::negative_len(self) {
            Existence::Negative
        } else {
            Existence::Nonexistent
        }
    }

    fn assert_size(&mut self, size: isize) {
        if size >= 0 {
            for _ in Self::positive_len(self)..=size as usize {
                Self::push_positive(self, U::default())
            }
        } else {
            for _ in Self::negative_len(self)..size.abs() as usize {
                Self::push_negative(self, U::default())
            }
        }
    }

    fn positive_len(&self) -> usize;
    fn negative_len(&self) -> usize;

    fn push_positive(&mut self, item: U);
    fn push_negative(&mut self, item: U);
}

#[derive(Debug)]
struct NegativeIndexVec<T> {
    positive: Vec<Option<T>>,
    negative: Vec<Option<T>>,
}

impl<T> NegativeIndexed<Option<T>> for NegativeIndexVec<T> {
    fn positive_len(&self) -> usize {
        self.positive.len()
    }

    fn negative_len(&self) -> usize {
        self.negative.len()
    }

    fn push_positive(&mut self, item: Option<T>) {
        self.positive.push(item);
    }

    fn push_negative(&mut self, item: Option<T>) {
        self.negative.push(item);
    }
}

impl<T> NegativeIndexVec<T> {
    pub fn new() -> Self {
        Self {
            positive: vec![],
            negative: vec![],
        }
    }

    pub fn set(&mut self, index: isize, item: T) {
        self.assert_size(index);

        if index >= 0 {
            self.positive[index as usize] = Some(item);
        } else {
            self.negative[index.abs() as usize - 1] = Some(item);
        }
    }

    pub fn get(&self, index: isize) -> Option<&T> {
        match self.existence(index) {
            Existence::Positive => self.positive[index as usize].as_ref(),
            Existence::Negative => self.negative[(index.abs() as usize) - 1].as_ref(),
            Existence::Nonexistent => None,
        }
    }

    pub fn get_mut(&mut self, index: isize) -> Option<&mut T> {
        match self.existence(index) {
            Existence::Positive => self.positive[index as usize].as_mut(),
            Existence::Negative => self.negative[index.abs() as usize - 1].as_mut(),
            Existence::Nonexistent => None,
        }
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    positive: Vec<Option<NegativeIndexVec<T>>>,
    negative: Vec<Option<NegativeIndexVec<T>>>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl<T> NegativeIndexed<Option<NegativeIndexVec<T>>> for Grid<T> {
    fn positive_len(&self) -> usize {
        self.positive.len()
    }

    fn negative_len(&self) -> usize {
        self.negative.len()
    }

    fn push_positive(&mut self, item: Option<NegativeIndexVec<T>>) {
        self.positive.push(item);
    }

    fn push_negative(&mut self, item: Option<NegativeIndexVec<T>>) {
        self.negative.push(item);
    }
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self {
            positive: vec![],
            negative: vec![],
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        }
    }

    pub fn min_x(&self) -> isize {
        self.min_x
    }

    pub fn max_x(&self) -> isize {
        self.max_x
    }

    pub fn min_y(&self) -> isize {
        self.min_y
    }

    pub fn max_y(&self) -> isize {
        self.max_y
    }

    fn assert_existence(&mut self, x: isize) {
        if x >= 0 && self.positive[x as usize].is_none() {
            self.positive[x as usize] = Some(NegativeIndexVec::new());
        } else if x < 0 && self.negative[x.abs() as usize - 1].is_none() {
            self.negative[x.abs() as usize - 1] = Some(NegativeIndexVec::new());
        }
    }

    fn update_boundaries(&mut self, x: isize, y: isize) {
        if x < self.min_x {
            self.min_x = x;
        } else if x > self.max_x {
            self.max_x = x;
        }

        if y < self.min_y {
            self.min_y = y;
        } else if y > self.max_y {
            self.max_y = y;
        }
    }

    pub fn set(&mut self, x: isize, y: isize, item: T) {
        self.assert_size(x);
        self.assert_existence(x);
        self.update_boundaries(x, y);

        if x >= 0 {
            self.positive[x as usize].as_mut().unwrap().set(y, item);
        } else {
            self.negative[x.abs() as usize - 1]
                .as_mut()
                .unwrap()
                .set(y, item);
        }
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        match self.existence(x) {
            Existence::Positive => self.positive[x as usize].as_ref().unwrap().get(y),
            Existence::Negative => self.negative[x.abs() as usize - 1].as_ref().unwrap().get(y),
            Existence::Nonexistent => None,
        }
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        match self.existence(x) {
            Existence::Positive => self.positive[x as usize].as_mut().unwrap().get_mut(y),
            Existence::Negative => self.negative[x.abs() as usize - 1]
                .as_mut()
                .unwrap()
                .get_mut(y),
            Existence::Nonexistent => None,
        }
    }
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Grid, NegativeIndexVec};

    #[test]
    fn negative_vec() {
        let mut neg_vec = NegativeIndexVec::new();

        for i in -10..=10 {
            neg_vec.set(i, i);
        }

        for i in -10..=10 {
            assert_eq!(*neg_vec.get(i).unwrap(), i);
        }
    }

    #[test]
    fn negative_grid() {
        let mut grid = Grid::new();

        for x in -10..=10 {
            for y in -10..=10 {
                grid.set(x, y, x + y);
            }
        }

        for x in grid.min_x()..=grid.max_x() {
            for y in grid.min_y()..=grid.max_y() {
                assert_eq!(*grid.get(x, y).unwrap(), x + y);
            }
        }
    }
}
