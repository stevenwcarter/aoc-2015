pub mod template;

pub fn presents_delivered_to_house_part1(input: usize) -> usize {
    let sqrt_n = (input as f64).sqrt() as usize;

    (1..=sqrt_n)
        .filter(|n| input % n == 0)
        .map(|n| {
            if n != input / n {
                (input / n) * 10 + n * 10
            } else {
                n * 10
            }
        })
        .sum()
}
pub fn presents_delivered_to_house_part2(input: usize) -> usize {
    let sqrt_n = (input as f64).sqrt() as usize;

    (1..=sqrt_n)
        .filter(|n| input % n == 0)
        .map(|n| {
            let upper_n = input / n;
            if n != upper_n && upper_n * 50 > input {
                upper_n * 11 + n * 11
            } else if n * 50 > input {
                n * 11
            } else {
                0
            }
        })
        .sum()
}

// Use this file to add helper functions and additional modules.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SmallPoint {
    pub x: u16,
    pub y: u16,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl From<(usize, usize)> for SmallPoint {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as u16,
            y: value.1 as u16,
        }
    }
}
impl From<(u32, u32)> for SmallPoint {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0 as u16,
            y: value.1 as u16,
        }
    }
}
impl From<(u16, u16)> for SmallPoint {
    fn from(value: (u16, u16)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as u32,
            y: value.1 as u32,
        }
    }
}
impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl SmallPoint {
    pub fn up(&self, range: Option<u16>) -> Option<Self> {
        self.up_n(1, range)
    }
    pub fn left(&self, range: Option<u16>) -> Option<Self> {
        self.left_n(1, range)
    }
    pub fn right(&self, range: Option<u16>) -> Option<Self> {
        self.right_n(1, range)
    }
    pub fn down(&self, range: Option<u16>) -> Option<Self> {
        self.down_n(1, range)
    }
    pub fn udlr(&self, udlr: [u16; 4]) -> Vec<Self> {
        self.udlr_unfiltered(udlr)
            .iter()
            .filter_map(|p| *p)
            .collect()
    }
    pub fn udlr_unfiltered(&self, udlr: [u16; 4]) -> [Option<Self>; 4] {
        [
            self.up(Some(udlr[0])),
            self.down(Some(udlr[1])),
            self.left(Some(udlr[2])),
            self.right(Some(udlr[3])),
        ]
    }
    pub fn up_n(&self, offset: u16, range: Option<u16>) -> Option<Self> {
        if self.y >= offset && self.y - offset >= range.unwrap_or(0) {
            Some((self.x, self.y - offset).into())
        } else {
            None
        }
    }
    pub fn down_n(&self, offset: u16, range: Option<u16>) -> Option<Self> {
        if self.y + offset < range.unwrap_or(u16::MAX) {
            Some((self.x, self.y + offset).into())
        } else {
            None
        }
    }
    pub fn left_n(&self, offset: u16, range: Option<u16>) -> Option<Self> {
        if self.x >= offset && self.x - offset >= range.unwrap_or(0) {
            Some((self.x - offset, self.y).into())
        } else {
            None
        }
    }
    pub fn right_n(&self, offset: u16, range: Option<u16>) -> Option<Self> {
        if self.x + offset < range.unwrap_or(u16::MAX) {
            Some((self.x + offset, self.y).into())
        } else {
            None
        }
    }
    pub fn up_right(&self, range_x: Option<u16>, range_y: Option<u16>) -> Option<Self> {
        match self.up(range_y) {
            Some(point) => point.right(range_x),
            None => None,
        }
    }
    pub fn down_right(&self, range_x: Option<u16>, range_y: Option<u16>) -> Option<Self> {
        match self.down(range_y) {
            Some(point) => point.right(range_x),
            None => None,
        }
    }
    pub fn up_left(&self, range_x: Option<u16>, range_y: Option<u16>) -> Option<Self> {
        match self.up(range_y) {
            Some(point) => point.left(range_x),
            None => None,
        }
    }
    pub fn down_left(&self, range_x: Option<u16>, range_y: Option<u16>) -> Option<Self> {
        match self.down(range_y) {
            Some(point) => point.left(range_x),
            None => None,
        }
    }
    // let up_left = y > 0 && x > 0 && grid.contains(&(x - 1, y - 1));
    // let up_right = y > 0 && grid.contains(&(x + 1, y - 1));
    // let down_left = x > 0 && grid.contains(&(x - 1, y + 1));
    // let down_right = grid.contains(&(x + 1, y + 1));
}
impl Point {
    pub fn up(&self, range: Option<u32>) -> Option<Self> {
        self.up_n(1, range)
    }
    pub fn left(&self, range: Option<u32>) -> Option<Self> {
        self.left_n(1, range)
    }
    pub fn right(&self, range: Option<u32>) -> Option<Self> {
        self.right_n(1, range)
    }
    pub fn down(&self, range: Option<u32>) -> Option<Self> {
        self.down_n(1, range)
    }
    pub fn udlr(&self, udlr: [u32; 4]) -> Vec<Self> {
        self.udlr_unfiltered(udlr)
            .iter()
            .filter_map(|p| *p)
            .collect()
    }
    pub fn udlr_unfiltered(&self, udlr: [u32; 4]) -> [Option<Self>; 4] {
        [
            self.up(Some(udlr[0])),
            self.down(Some(udlr[1])),
            self.left(Some(udlr[2])),
            self.right(Some(udlr[3])),
        ]
    }
    pub fn up_n(&self, offset: u32, range: Option<u32>) -> Option<Self> {
        if self.y >= offset && self.y - offset >= range.unwrap_or(0) {
            Some((self.x, self.y - offset).into())
        } else {
            None
        }
    }
    pub fn down_n(&self, offset: u32, range: Option<u32>) -> Option<Self> {
        if self.y + offset < range.unwrap_or(u32::MAX) {
            Some((self.x, self.y + offset).into())
        } else {
            None
        }
    }
    pub fn left_n(&self, offset: u32, range: Option<u32>) -> Option<Self> {
        if self.x >= offset && self.x - offset >= range.unwrap_or(0) {
            Some((self.x - offset, self.y).into())
        } else {
            None
        }
    }
    pub fn right_n(&self, offset: u32, range: Option<u32>) -> Option<Self> {
        if self.x + offset < range.unwrap_or(u32::MAX) {
            Some((self.x + offset, self.y).into())
        } else {
            None
        }
    }
    pub fn up_right(&self, range_x: Option<u32>, range_y: Option<u32>) -> Option<Self> {
        match self.up(range_y) {
            Some(point) => point.right(range_x),
            None => None,
        }
    }
    pub fn down_right(&self, range_x: Option<u32>, range_y: Option<u32>) -> Option<Self> {
        match self.down(range_y) {
            Some(point) => point.right(range_x),
            None => None,
        }
    }
    pub fn up_left(&self, range_x: Option<u32>, range_y: Option<u32>) -> Option<Self> {
        match self.up(range_y) {
            Some(point) => point.left(range_x),
            None => None,
        }
    }
    pub fn down_left(&self, range_x: Option<u32>, range_y: Option<u32>) -> Option<Self> {
        match self.down(range_y) {
            Some(point) => point.left(range_x),
            None => None,
        }
    }
    // let up_left = y > 0 && x > 0 && grid.contains(&(x - 1, y - 1));
    // let up_right = y > 0 && grid.contains(&(x + 1, y - 1));
    // let down_left = x > 0 && grid.contains(&(x - 1, y + 1));
    // let down_right = grid.contains(&(x + 1, y + 1));
}

// TODO: Add "one_up", etc. methods

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardinalDirection {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_moves() {
        let point = Point::from((10u32, 10));

        assert_eq!(point.up_n(1, None), Some((10u32, 9).into()));
        assert_eq!(point.up_n(10, None), Some((10u32, 0).into()));
        assert_eq!(point.down_n(1, None), Some((10u32, 11).into()));
        assert_eq!(point.up_n(11, None), None);
        assert_eq!(point.down_n(10, None), Some((10u32, 20).into()));
        assert_eq!(point.right_n(10, None), Some((20u32, 10).into()));
        assert_eq!(point.left_n(11, None), None);
        assert_eq!(point.left_n(10, None), Some((0u32, 10).into()));
        assert_eq!(point.left_n(1, None), Some((9u32, 10).into()));
    }
    #[test]
    fn test_house_1() {
        assert_eq!(presents_delivered_to_house_part1(1), 10);
    }
    #[test]
    fn test_house_1_part2() {
        assert_eq!(presents_delivered_to_house_part2(1), 11);
    }
    #[test]
    fn test_problem_1() {
        assert_eq!(presents_delivered_to_house_part1(776160), 33611760);
    }
    #[test]
    fn test_house_part1_2() {
        assert_eq!(presents_delivered_to_house_part1(2), 30);
    }
    #[test]
    fn test_house_part1_2_part2() {
        assert_eq!(presents_delivered_to_house_part2(2), 33);
    }
    #[test]
    fn test_house_part1_3() {
        assert_eq!(presents_delivered_to_house_part1(3), 40);
    }
    #[test]
    fn test_house_part1_4() {
        assert_eq!(presents_delivered_to_house_part1(4), 70);
    }
    #[test]
    fn test_house_part1_5() {
        assert_eq!(presents_delivered_to_house_part1(5), 60);
    }
    #[test]
    fn test_house_part1_6() {
        assert_eq!(presents_delivered_to_house_part1(6), 120);
    }
    #[test]
    fn test_house_part1_7() {
        assert_eq!(presents_delivered_to_house_part1(7), 80);
    }
    #[test]
    fn test_house_part1_8() {
        assert_eq!(presents_delivered_to_house_part1(8), 150);
    }
    #[test]
    fn test_house_part1_9() {
        assert_eq!(presents_delivered_to_house_part1(9), 130);
    }
    #[test]
    fn test_house_part1_9_part2() {
        assert_eq!(presents_delivered_to_house_part2(9), 143);
    }
}
