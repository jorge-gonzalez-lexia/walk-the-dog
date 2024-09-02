#[derive(Clone, Copy, Default)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

#[derive(Default)]
pub struct Rect {
    pub position: Point,
    pub width: i16,
    pub height: i16,
}

impl Rect {
    pub const fn new(position: Point, width: i16, height: i16) -> Self {
        Rect {
            position,
            width,
            height,
        }
    }

    pub const fn new_from_x_y(x: i16, y: i16, width: i16, height: i16) -> Self {
        Rect::new(Point { x, y }, width, height)
    }

    pub fn bottom(&self) -> i16 {
        self.y() + self.height
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.left() < other.right()
            && self.right() >= other.left()
            && self.top() < other.bottom()
            && self.bottom() >= other.top()
    }

    pub fn left(&self) -> i16 {
        self.x()
    }

    pub fn right(&self) -> i16 {
        self.x() + self.width
    }

    pub fn set_x(&mut self, x: i16) {
        self.position.x = x;
    }

    pub fn top(&self) -> i16 {
        self.y()
    }

    pub fn x(&self) -> i16 {
        self.position.x
    }

    pub fn y(&self) -> i16 {
        self.position.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_rects_that_intersect_on_the_left() {
        let rect1 = Rect {
            position: Point { x: 10, y: 10 },
            width: 10,
            height: 10,
        };
        let rect2 = Rect {
            position: Point { x: 0, y: 10 },
            width: 10,
            height: 10,
        };

        assert!(rect2.intersects(&rect1));
    }

    #[test]
    fn two_rects_that_intersect_on_the_bottom() {
        let rect1 = Rect {
            position: Point { x: 10, y: 10 },
            width: 10,
            height: 10,
        };
        let rect2 = Rect {
            position: Point { x: 10, y: 0 },
            width: 10,
            height: 10,
        };

        assert!(rect2.intersects(&rect1));
    }

    #[test]
    fn one_rect_fully_below_another() {
        let rect1 = Rect {
            position: Point { x: 10, y: 10 },
            width: 100,
            height: 10,
        };
        let rect2 = Rect {
            position: Point { x: 10, y: 20 },
            width: 100,
            height: 10,
        };

        assert!(!rect2.intersects(&rect1));
    }

    #[test]
    fn one_rect_fully_left_of_another() {
        let rect1 = Rect {
            position: Point { x: 20, y: 10 },
            width: 10,
            height: 10,
        };
        let rect2 = Rect {
            position: Point { x: 0, y: 10 },
            width: 10,
            height: 10,
        };

        assert!(!rect2.intersects(&rect1));
    }
}
