use std::mem::replace;

type Predicate<T> = Box<dyn Fn(&T) -> bool>;
type GroupedPredicates<T> = Vec<Predicate<T>>;

enum Connector<T> {
    And(GroupedPredicates<T>, Option<Box<Connector<T>>>),
    Or(GroupedPredicates<T>, Option<Box<Connector<T>>>),
    Not(GroupedPredicates<T>, Option<Box<Connector<T>>>),
}

impl<T> Connector<T> {
    fn apply(&self, value: &T) -> bool {
        match self {
            Connector::And(items, connector) => {
                let result = connector.as_ref().is_none_or(|c| c.apply(value));

                if !result {
                    return false;
                }

                items.iter().all(|f| f(value))
            }
            Connector::Or(items, connector) => {
                let result = connector.as_ref().is_none_or(|c| c.apply(value));

                if result {
                    return true;
                }

                items.iter().all(|f| f(value))
            }
            Connector::Not(items, connector) => {
                let matched = items.iter().all(|f| !f(value));

                match connector {
                    Some(items) => items.apply(value),
                    None => matched,
                }
            }
        }
    }
}

pub struct Filters<T> {
    _filters: Connector<T>,
}

impl<T> Filters<T> {
    pub fn new() -> Self {
        Self {
            _filters: Connector::And(Vec::new(), None),
        }
    }

    pub fn apply(&self, value: &T) -> bool {
        self._filters.apply(value)
    }

    pub fn push<F>(&mut self, predicate: F) -> &mut Self
    where
        F: Fn(&T) -> bool + 'static,
    {
        match &mut self._filters {
            Connector::And(items, _) | Connector::Or(items, _) => {
                items.push(Box::new(predicate));
            }
            Connector::Not(_, connector) => {
                if let Some(c) = connector {
                    match &**c {
                        Connector::And(items, connector) => todo!(),
                        Connector::Or(items, connector) => todo!(),
                        Connector::Not(items, connector) => todo!(),
                    }
                }
            }
        }

        self
    }

    pub fn not(&mut self) -> &mut Self {
        let old_filters = replace(&mut self._filters, Connector::And(Vec::new(), None));

        self._filters = Connector::Not(Vec::new(), Some(Box::new(old_filters)));

        self
    }

    pub fn and(&mut self) -> &mut Self {
        let old_filters = replace(&mut self._filters, Connector::And(Vec::new(), None));

        self._filters = Connector::And(Vec::new(), Some(Box::new(old_filters)));

        self
    }

    pub fn or(&mut self) -> &mut Self {
        let old_filters = replace(&mut self._filters, Connector::And(Vec::new(), None));

        self._filters = Connector::Or(Vec::new(), Some(Box::new(old_filters)));

        self
    }
}
