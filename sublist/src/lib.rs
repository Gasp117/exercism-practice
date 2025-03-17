#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    use Comparison::*;
    match (first.len(), second.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n => if first.windows(n).any(|v| v == second) {Superlist} else {Unequal},
        (m, n) if m < n => if second.windows(m).any(|v| v == first) {Sublist} else {Unequal},
        (_, _) => if first == second {Equal} else {Unequal},
    }
}