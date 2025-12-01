pub struct SortedVecSet<T>(Vec<T>);

impl<T> Default for SortedVecSet<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self(Vec::default())
    }
}

impl<T> SortedVecSet<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    pub fn insert(&mut self, value: T) -> bool {
        match self.0.binary_search(&value) {
            Ok(_) => false,
            Err(index) => {
                self.0.insert(index, value);
                true
            }
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.0.binary_search(value).is_ok()
    }

    pub fn remove(&mut self, value: &T) -> bool {
        match self.0.binary_search(value) {
            Ok(index) => {
                self.0.remove(index);
                true
            }
            Err(_) => false,
        }
    }
}

impl<T> AsRef<[T]> for SortedVecSet<T>
where
    T: Ord,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> std::ops::Deref for SortedVecSet<T>
where
    T: Ord,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> IntoIterator for SortedVecSet<T> {
    type IntoIter = std::vec::IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
