#[derive(Debug, Clone)]
pub struct Vector2d {
    pub x: i32,
    pub y: i32,
}

impl Vector2d {
    // Operations
    pub fn add(&self, other: &Vector2d) -> Vector2d {
        return Vector2d {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }

    pub fn sub(&self, other: &Vector2d) -> Vector2d {
        return Vector2d {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
    pub fn mul(&self, other: &Vector2d) -> Vector2d {
        return Vector2d {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }

    pub fn div(&self, other: &Vector2d) -> Vector2d {
        return Vector2d {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }

    // restrict within bounds
    pub fn restrict_to_bounds(&self, min: &Vector2d, max: &Vector2d) -> Vector2d {
        return Vector2d {
            x: self.x.max(min.x).min(max.x),
            y: self.y.max(min.y).min(max.y),
        };
    }

    // Generate unique id
    pub fn id(&self) -> String {
        return format!("X{}/Y{}", self.x, self.y);
    }

    // statics
    pub fn up() -> Vector2d {
        return Vector2d { x: 0, y: 1 };
    }
    pub fn down() -> Vector2d {
        return Vector2d { x: 0, y: -1 };
    }
    pub fn left() -> Vector2d {
        return Vector2d { x: -1, y: 0 };
    }
    pub fn right() -> Vector2d {
        return Vector2d { x: 1, y: 0 };
    }
    pub fn zero() -> Vector2d {
        return Vector2d { x: 0, y: 0 };
    }
}
