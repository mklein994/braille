use super::Point;

pub struct CartesianBound {
    pub min: f64,
    pub max: f64,
}

impl CartesianBound {
    fn new(min: f64, max: f64) -> Self {
        debug_assert!(min < max);
        Self { min, max }
    }
}

impl Default for CartesianBound {
    fn default() -> Self {
        Self {
            min: f64::MAX,
            max: f64::MIN,
        }
    }
}

pub struct CartesianBounds {
    pub x: CartesianBound,
    pub y: CartesianBound,
}

#[derive(Clone, Copy)]
pub struct CartesianBoundsBuilder {
    x_min: BuilderBound,
    x_max: BuilderBound,
    y_min: BuilderBound,
    y_max: BuilderBound,
}

impl Default for CartesianBoundsBuilder {
    fn default() -> Self {
        Self {
            x_min: BuilderBound::new(None, CmpKind::Min),
            x_max: BuilderBound::new(None, CmpKind::Max),
            y_min: BuilderBound::new(None, CmpKind::Min),
            y_max: BuilderBound::new(None, CmpKind::Max),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum CmpKind {
    Min,
    Max,
}

#[derive(Clone, Copy)]
struct BuilderBound {
    value: f64,
    kind: CmpKind,
    provided: bool,
}

impl BuilderBound {
    fn new(value: Option<f64>, kind: CmpKind) -> Self {
        match (value, kind) {
            (Some(value), kind) => Self {
                value,
                kind,
                provided: true,
            },
            (None, kind @ CmpKind::Min) => Self {
                value: f64::MAX,
                kind,
                provided: false,
            },
            (None, kind @ CmpKind::Max) => Self {
                value: f64::MIN,
                kind,
                provided: false,
            },
        }
    }

    fn maybe_update(&mut self, value: f64) {
        if self.provided {
            return;
        }

        match self.kind {
            CmpKind::Min => {
                self.value = self.value.min(value);
            }
            CmpKind::Max => {
                self.value = self.value.max(value);
            }
        }
    }
}

impl CartesianBoundsBuilder {
    pub fn x_min(&mut self, min: f64) {
        self.x_min.provided = true;
        self.x_min.value = min;
    }

    pub fn x_max(&mut self, max: f64) {
        self.x_max.provided = true;
        self.x_max.value = max;
    }

    pub fn y_min(&mut self, min: f64) {
        self.y_min.provided = true;
        self.y_min.value = min;
    }

    pub fn y_max(&mut self, max: f64) {
        self.y_max.provided = true;
        self.y_max.value = max;
    }

    pub fn update_from_points(self, points: &[Point]) -> CartesianBounds {
        let Self {
            mut x_min,
            mut x_max,
            mut y_min,
            mut y_max,
        } = self;
        for point in points {
            x_min.maybe_update(point.x);
            x_max.maybe_update(point.x);
            y_min.maybe_update(point.y);
            y_max.maybe_update(point.y);
        }

        CartesianBounds::new(x_min.value, x_max.value, y_min.value, y_max.value)
    }
}

impl CartesianBounds {
    pub fn builder() -> CartesianBoundsBuilder {
        CartesianBoundsBuilder::default()
    }

    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Self {
        Self {
            x: CartesianBound::new(x_min, x_max),
            y: CartesianBound::new(y_min, y_max),
        }
    }
}
