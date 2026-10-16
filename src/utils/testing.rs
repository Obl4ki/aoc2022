use std::collections::HashMap;

#[allow(unused)]
pub fn assert_equal_hashmaps<K, V>(expected: &HashMap<K, V>, actual: &HashMap<K, V>)
where
    K: std::fmt::Debug + Eq + std::hash::Hash,
    V: std::fmt::Debug + PartialEq,
{
    // Assert that both HashMaps are the same, lenght and key-value pairs. Output a message only with the keys that are present in one but not the other.
    let mut diffs = Vec::new();
    for key in expected.keys() {
        if !actual.contains_key(key) {
            diffs.push(format!(
                "Key {:?} is present in expected but not in actual",
                key
            ));
        }
        if actual.get(key) != expected.get(key) {
            diffs.push(format!(
                "Key {:?} has different values: expected {:?}, actual {:?}",
                key,
                expected.get(key),
                actual.get(key)
            ));
        }
    }
    for key in actual.keys() {
        if !expected.contains_key(key) {
            diffs.push(format!(
                "Key {:?} is present in actual but not in expected",
                key
            ));
        }
        if expected.get(key) != actual.get(key) {
            diffs.push(format!(
                "Key {:?} has different values: expected {:?}, actual {:?}",
                key,
                expected.get(key),
                actual.get(key)
            ));
        }
    }
    if !diffs.is_empty() {
        panic!("HashMaps are not equal:\n{}", diffs.join("\n"));
    }

    if expected.len() != actual.len() {
        panic!(
            "HashMaps are not equal: expected length {}, actual length {}",
            expected.len(),
            actual.len()
        );
    }

    assert_eq!(
        expected, actual,
        "HashMaps are not equal: expected {:?}, actual {:?}",
        expected, actual
    );
}
