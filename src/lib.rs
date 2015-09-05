/// Generic Trie implementation
///
/// Doesn't feature any Patricia optimizations (each node has only a single key)
///
/// # Examples
///
/// ```
/// use trie;
///
/// let mut t: trie::Trie<char, String> = trie::Trie::new_empty();
/// t.insert("abc".chars(), "foobar".to_string()).ok();
///
/// let query = "abcd";
/// if let Some(value) = t.search(query.chars()) {
///     assert_eq!(value, "foobar");
/// }
/// ```

#[derive(Debug)]
pub struct Trie<K, D> {
    children: Vec<Trie<K, D>>,
    key: Option<K>,
    data: Option<D>,
}

pub type ErrType = Result<(), &'static str>;

impl<K: PartialEq + Copy, D> Trie<K, D> {

    /// Construct a new, empty Trie
    pub fn new_empty() -> Trie<K, D> {
        Trie {
            children: vec![],
            key: None,
            data: None
        }
    }

    /// Insert a new value into the Trie through an iterator.
    ///
    /// Inserting a key that is already present is illegal.
    ///
    /// key_elems should be an Iterator over whatever the Key Type is (e.g., an iterator of `char`
    ///     if the KeyType is `char`)
    /// data will be Moved into the Trie
    pub fn insert_iter<F: Iterator<Item=K>>(&mut self, mut key_elems: F, data: D) -> ErrType {
        let this_key: Option<K> = key_elems.next();

        if let Some(this_key_value) = this_key {
            for mut child in self.children.iter_mut() {
                // If the keys match
                if let Some(child_key_value) = child.key {
                    if child_key_value == this_key_value {
                        // insert into the child!
                        return child.insert(key_elems, data);
                    }
                }
            }
            // Guess we have to make a new one
            let mut new_child = Trie {
                children: vec![],
                key: Some(this_key_value),
                data: None,
            };
            let res = new_child.insert(key_elems, data);
            self.children.push(new_child);
            return res;
        } else {
            return match self.data {
                None => {
                    self.data = Some(data);
                    Ok(())
                }
                Some(_) => {
                    Err("key already present!")
                }
            }
        }
    }

    /// Insert a new value into the Trie through an iterator
    ///
    /// Syntactic sugar for [self.insert_iter]
    pub fn insert<F: IntoIterator<Item=K>>(&mut self, key: F, data: D) -> ErrType {
        return self.insert_iter(key.into_iter(), data);
    }

    /// Search for the longest match in the Trie
    pub fn search_iter<F: Iterator<Item=K>>(&self, mut key_elems: F) -> Option<&D> {
        let this_key: Option<K> = key_elems.next();

        // Does the key we got out of the iterator even do anything?
        match this_key {
            Some(this_key_value) => {
                // walk through each children looking for one matching the key
                for child in self.children.iter() {
                   // if the keys match
                    if let Some(child_key_value) = child.key {
                        if child_key_value == this_key_value {
                            // recurse
                            return child.search(key_elems)
                        }
                    }
                }
                // If we didn't find anything recursively, but we have a data,
                // then *we* must be the longest match!
                match self.data {
                    Some(ref data_val) => return Some(&data_val),
                    None => None
                }
            },
            None => match self.data {
                Some(ref data_val) => return Some(&data_val),
                None => return None,
            }
        }
    }

    /// Search for a value in the Trie given an interator
    ///
    /// Syntactic sugar for [self.search_iter]
    pub fn search<F: IntoIterator<Item=K>>(&self, key: F) -> Option<&D> {
        return self.search_iter(key.into_iter());
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn create_trie() {
        let mut t = Trie::new_empty();

        assert_eq!(t.insert("abc".chars(), "foobar".to_string()), Ok(()));

        assert_eq!(t.search("ab".chars()), None);

        let res = t.search("abc".chars());
        assert!(res != None);
        if let Some(value) = res {
            assert_eq!(value, "foobar")
        }
    }

    #[test]
    fn test_longest_match() {
        let mut t = Trie::new_empty();
        assert_eq!(t.insert("abc".chars(), "object 1".to_string()), Ok(()));
        assert_eq!(t.insert("ab".chars(), "object 2".to_string()), Ok(()));

        let res1 = t.search("abc".chars());

        assert!(res1 != None);
        if let Some(value) = res1 {
            assert_eq!(value, "object 1");
        }

        let res2 = t.search("abcdef".chars());

        assert!(res2 != None);
        if let Some(value) = res2 {
            assert_eq!(value, "object 1");
        }

        let res3 = t.search("ab".chars());

        assert!(res3 != None);
        if let Some(value) = res3 {
            assert_eq!(value, "object 2");
        }
    }

    #[test]
    fn test_ints_and_vectors() {
        let mut t: Trie<i32, i32> = Trie::new_empty();


        assert_eq!(t.insert(vec![1, 2], 10), Ok(()));
        assert_eq!(t.insert(vec![1, 2, 3], 20), Ok(()));

        assert_eq!(t.search(vec![1]), None);

        if let Some(value) = t.search(vec![1, 2, 3]) {
            assert_eq!(value, &20);
        } else {
            assert!(false);
        }

        if let Some(value) = t.search(vec![1, 2, 4]) {
            assert_eq!(value, &10);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_double_insert_fails() {
        let mut t = Trie::new_empty();

        assert!(t.insert("ab".chars(), 1).is_ok());
        assert!(t.insert("ab".chars(), 1).is_err());
    }
}
