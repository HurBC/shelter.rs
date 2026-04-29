type Predicate<T> = Box<dyn Fn(&T) -> bool>;
type Filters<T> = Vec<Predicate<T>>;

pub enum Connector<T> {
    And(Filters<T>, Option<Box<Connector<T>>>),
    Or(Filters<T>, Option<Box<Connector<T>>>),
}

impl<T> Connector<T> {
    pub fn apply(&self, value: &T) -> bool {
        match self {
            Connector::And(items, conector) => {
                let has_match = items.iter().all(|f| f(value));

                if !has_match {
                    return false;
                }

                match conector {
                    Some(conector) => conector.apply(value),
                    None => has_match,
                }
            }
            Connector::Or(items, conector) => {
                let has_match = items.iter().any(|f| f(value));

                if has_match {
                    return true;
                }

                match conector {
                    Some(conector) => conector.apply(value),
                    None => has_match,
                }
            }
        }
    }
}
