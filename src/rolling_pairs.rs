pub struct RollingPairsState<'a, Iter>
where
    Iter: Iterator,
    Iter::Item: Clone,
{
    prev_item: Option<Iter::Item>,
    iter: &'a mut Iter,
}

pub trait RollingPairs<Iter>
where
    Iter: Iterator,
    Iter::Item: Clone,
{
    fn rolling_pairs(&mut self) -> RollingPairsState<Iter>;
}

impl<Iter> RollingPairs<Iter> for Iter
where
    Iter: Iterator,
    Iter::Item: Clone,
{
    fn rolling_pairs(&mut self) -> RollingPairsState<Iter> {
        let prev_item = self.next();

        RollingPairsState {
            prev_item,
            iter: self,
        }
    }
}

impl<'a, Iter> Iterator for RollingPairsState<'a, Iter>
where
    Iter: Iterator,
    Iter::Item: Clone,
{
    type Item = (Iter::Item, Iter::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.prev_item.clone();
        let next = self.iter.next();

        self.prev_item = next.clone();

        map_options_into_tuple(prev, next)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

fn map_options_into_tuple<A, B>(x: Option<A>, y: Option<B>) -> Option<(A, B)> {
    match (x, y) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    #[test]
    fn returns_pairs_of_adjacent_elements() {
        let res = (1..5).rolling_pairs().collect::<Vec<(u32, u32)>>();

        assert_eq!(res, vec!((1, 2), (2, 3), (3, 4)));
    }

    #[test]
    fn returns_nothing_for_iterators_of_one_item() {
        let chars = vec!['a'];

        let res = chars
            .iter()
            .rolling_pairs()
            .collect::<Vec<(&char, &char)>>();

        assert_eq!(res, vec!());
    }

    #[test]
    fn returns_nothing_for_an_empty_iterator() {
        let res = iter::empty().rolling_pairs().collect::<Vec<(u32, u32)>>();

        assert_eq!(res, vec!());
    }

    #[test]
    fn provides_an_accurate_size_hint_in_the_initial_state() {
        let size_hint = (1..10).rolling_pairs().size_hint();

        assert_eq!(size_hint, (8, Some(8)));
    }

    #[test]
    fn provides_an_accurate_size_hint_for_iterators_of_one_item() {
        let size_hint = vec![1].iter().rolling_pairs().size_hint();

        assert_eq!(size_hint, (0, Some(0)));
    }

    #[test]
    fn provides_an_accurate_size_hint_for_empty_iterators() {
        let size_hint = iter::empty::<u32>().rolling_pairs().size_hint();

        assert_eq!(size_hint, (0, Some(0)));
    }

    #[test]
    fn provides_an_accurate_size_hint_after_iteration_has_started() {
        let mut digits = 1..10;
        let mut iter = digits.rolling_pairs();

        iter.next();

        assert_eq!(iter.size_hint(), (7, Some(7)));
    }
}
