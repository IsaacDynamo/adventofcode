pub static DIR: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

#[derive(Debug, Clone)]
pub struct Grid<T> {
    size: (i64, i64),
    data: Vec<T>,
}

impl<T: Copy> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let size = (data[0].len() as i64, data.len() as i64);
        let data: Vec<T> = data.iter().flat_map(|line| line.iter().copied()).collect();
        assert_eq!(data.len() as i64, size.0 * size.1);
        Grid { size, data }
    }

    pub fn size(&self) -> (i64, i64) {
        self.size
    }

    pub fn get(&self, x: i64, y: i64) -> Option<T> {
        if 0 <= x && x < self.size.0 && 0 <= y && y < self.size.1 {
            Some(self.data[(self.size.0 * y + x) as usize])
        } else {
            None
        }
    }

    pub fn get_ref(&self, x: i64, y: i64) -> Option<&T> {
        if 0 <= x && x < self.size.0 && 0 <= y && y < self.size.1 {
            Some(&self.data[(self.size.0 * y + x) as usize])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: i64, y: i64) -> Option<&mut T> {
        if 0 <= x && x < self.size.0 && 0 <= y && y < self.size.1 {
            Some(&mut self.data[(self.size.0 * y + x) as usize])
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (i64, i64, T)> + '_ {
        (0..self.size.1)
            .flat_map(move |y| (0..self.size.0).map(move |x| (x, y, self.get(x, y).unwrap())))
    }

    pub fn map<U>(&self, func: impl Fn(i64, i64, T) -> U) -> Grid<U> {
        let data = self.iter().map(|(x, y, v)| func(x, y, v)).collect();
        Grid {
            size: self.size,
            data,
        }
    }
}

impl Grid<char> {
    pub fn from_str(input: &str) -> Self {
        Grid::new(input.lines().map(|x| x.chars().collect()).collect())
    }
}