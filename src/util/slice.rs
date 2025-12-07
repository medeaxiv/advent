pub trait SliceExt {
    type Item;

    fn multi_index<const N: usize>(
        &self,
        indices: [usize; N],
    ) -> Result<[&Self::Item; N], MultiIndexError>;

    fn multi_index_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[&mut Self::Item; N], MultiIndexMutError>;
}

impl<T> SliceExt for [T] {
    type Item = T;

    fn multi_index<const N: usize>(
        &self,
        indices: [usize; N],
    ) -> Result<[&Self::Item; N], MultiIndexError> {
        multi_index(self, indices)
    }

    fn multi_index_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[&mut Self::Item; N], MultiIndexMutError> {
        multi_index_mut(self, indices)
    }
}

impl<T> SliceExt for Vec<T> {
    type Item = T;

    fn multi_index<const N: usize>(
        &self,
        indices: [usize; N],
    ) -> Result<[&Self::Item; N], MultiIndexError> {
        multi_index(self.as_slice(), indices)
    }

    fn multi_index_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[&mut Self::Item; N], MultiIndexMutError> {
        multi_index_mut(self.as_mut_slice(), indices)
    }
}

fn multi_index<const N: usize, T>(
    slice: &[T],
    indices: [usize; N],
) -> Result<[&T; N], MultiIndexError> {
    if let Some(err) = check_multi_index(slice.len(), &indices) {
        return Err(err);
    }

    let items = unsafe { multi_index_unchecked(slice, indices) };
    Ok(items)
}

fn check_multi_index(len: usize, indices: &[usize]) -> Option<MultiIndexError> {
    for &index in indices {
        if index >= len {
            return Some(MultiIndexError::OutOfBounds(index));
        }
    }

    None
}

unsafe fn multi_index_unchecked<const N: usize, T>(slice: &[T], indices: [usize; N]) -> [&T; N] {
    let slice_ptr = slice.as_ptr();
    indices.map(|index| {
        let item_ptr = unsafe { slice_ptr.add(index) };
        unsafe { item_ptr.as_ref().unwrap() }
    })
}

fn multi_index_mut<const N: usize, T>(
    slice: &mut [T],
    indices: [usize; N],
) -> Result<[&mut T; N], MultiIndexMutError> {
    if let Some(err) = check_multi_index_mut(slice.len(), &indices) {
        return Err(err);
    }

    let items = unsafe { multi_index_mut_unchecked(slice, indices) };
    Ok(items)
}

fn check_multi_index_mut(len: usize, indices: &[usize]) -> Option<MultiIndexMutError> {
    for i in 0..indices.len() {
        if i >= len {
            return Some(MultiIndexMutError::OutOfBounds(i));
        }

        for j in (i + 1)..indices.len() {
            if i == j {
                return Some(MultiIndexMutError::Alias(i));
            }
        }
    }

    None
}

unsafe fn multi_index_mut_unchecked<const N: usize, T>(
    slice: &mut [T],
    indices: [usize; N],
) -> [&mut T; N] {
    use std::ptr::NonNull;

    let slice_ptr = slice.as_mut_ptr();
    let slice_ptr = unsafe { NonNull::new_unchecked(slice_ptr) };
    indices.map(|index| {
        let mut item_ptr = unsafe { slice_ptr.add(index) };
        unsafe { item_ptr.as_mut() }
    })
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum MultiIndexError {
    #[error("index out of bounds")]
    OutOfBounds(usize),
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum MultiIndexMutError {
    #[error("index out of bounds")]
    OutOfBounds(usize),
    #[error("aliased index")]
    Alias(usize),
}
