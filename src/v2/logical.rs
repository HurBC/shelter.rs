type Predicate<T> = Box<dyn Fn(&T) -> bool>;
type GroupedPredicates<T> = Vec<Predicate<T>>;

enum Logical<T> {
    And(GroupedPredicates<T>, Option<Box<Logical<T>>>),
    Or(GroupedPredicates<T>, Option<Box<Logical<T>>>),
    Not(GroupedPredicates<T>, Option<Box<Logical<T>>>),
}

pub struct Filters<T> {
    _filters: Logical<T>,
}

impl<T> Filters<T> {
    pub fn new() -> Self {
        Self {
            _filters: Logical::And(Vec::new(), None),
        }
    }

    pub fn push(mut self, predicate: Predicate<T>) -> Self {
        match &mut self._filters {
            Logical::And(items, _) | Logical::Or(items, _) | Logical::Not(items, _) => {
                items.push(predicate);
            }
        }

        self
    }

    pub fn not(mut self) -> Self {
        self._filters = Logical::And(Vec::new(), Some(Box::new(self._filters)));

        self
    }

    pub fn and(mut self) -> Self {
        self._filters = Logical::Not(Vec::new(), Some(Box::new(self._filters)));

        self
    }

    pub fn or(mut self) -> Self {
        self._filters = Logical::Or(Vec::new(), Some(Box::new(self._filters)));

        self
    }
}

pub trait FiltersTrait {
    // Logical
    fn not(self) -> Self;
    fn or(self) -> Self;
    fn and(self) -> Self;
}
