// This is currently hard-coded to only work with arrays of length 5, but once
// RFC 2000 - const_generics is added to Rust this could easily be made to work
// with arrays of any length. For more info on const_generics see:
// - RFC: https://rust-lang.github.io/rfcs/2000-const-generics.html
// - GH Issue: https://github.com/rust-lang/rust/issues/44580

#[derive(Debug, Copy, Clone)]
pub struct Permutations<T>
where
    T: Copy,
{
    values: [T; 5],
    c: [usize; 5],
    i: usize,
    emitted_first_value: bool,
}

impl<T> Permutations<T>
where
    T: Copy,
{
    pub fn of(values: [T; 5]) -> Permutations<T> {
        Permutations {
            values,
            c: [0; 5],
            i: 0,
            emitted_first_value: false,
        }
    }
}

impl<T> Iterator for Permutations<T>
where
    T: Copy,
{
    type Item = [T; 5];

    fn next(&mut self) -> Option<Self::Item> {
        if !self.emitted_first_value {
            self.emitted_first_value = true;
            return Some(self.values);
        }

        while self.i < self.values.len() {
            if self.c[self.i] < self.i {
                let swap_index = if self.i % 2 == 0 { 0 } else { self.c[self.i] };
                let x = self.values[swap_index];
                self.values[swap_index] = self.values[self.i];
                self.values[self.i] = x;

                self.c[self.i] += 1;
                self.i = 0;

                return Some(self.values);
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_correct_number_of_permutations() {
        assert_eq!(
            Permutations::of(['a', 'b', 'c', 'd', 'e']).count(),
            5 * 4 * 3 * 2 * 1
        );
    }

    #[test]
    fn includes_the_first_value() {
        let original = ['a', 'b', 'c', 'd', 'e'];

        let all = Permutations::of(original).collect::<Vec<_>>();

        assert!(all.contains(&original));
    }

    #[test]
    fn includes_the_original_value_reversed() {
        let mut original = ['a', 'b', 'c', 'd', 'e'];
        original.reverse();

        let all = Permutations::of(original).collect::<Vec<_>>();

        assert!(all.contains(&original));
    }
}
