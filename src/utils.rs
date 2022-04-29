/// Utilities

pub fn uniq<I>(it: I) -> bool
where
    I: Iterator,
    I::Item: PartialEq,
{
    // HashMap would be a better choice if I::Item: Hash
    let mut seen: Vec<I::Item> = Vec::new();
    for i in it {
        if seen.iter().any(|x| x == &i) {
            return false;
        }
        seen.push(i);
    }
    true
}
