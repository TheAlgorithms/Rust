use std::ops::{Add, AddAssign, Sub, SubAssign};

/// A Fenwick Tree (also known as a Binary Indexed Tree) that supports efficient
/// prefix sum, range sum and point queries, as well as point updates.
///
/// The Fenwick Tree uses **1-based** indexing internally but presents a **0-based** interface to the user.
/// This design improves efficiency and simplifies both internal operations and external usage.
pub struct FenwickTree<T>
where
    T: Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Copy + Default,
{
    /// Internal storage of the Fenwick Tree. The first element (index 0) is unused
    /// to simplify index calculations, so the effective tree size is `data.len() - 1`.
    data: Vec<T>,
}

/// Enum representing the possible errors that can occur during FenwickTree operations.
#[derive(Debug, PartialEq, Eq)]
pub enum FenwickTreeError {
    /// Error indicating that an index was out of the valid range.
    IndexOutOfBounds,
    /// Error indicating that a provided range was invalid (e.g., left > right).
    InvalidRange,
}

impl<T> FenwickTree<T>
where
    T: Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Copy + Default,
{
    /// Creates a new Fenwick Tree with a specified capacity.
    ///
    /// The tree will have `capacity + 1` elements, all initialized to the default
    /// value of type `T`. The additional element allows for 1-based indexing internally.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The number of elements the tree can hold (excluding the extra element).
    ///
    /// # Returns
    ///
    /// A new `FenwickTree` instance.
    pub fn with_capacity(capacity: usize) -> Self {
        FenwickTree {
            data: vec![T::default(); capacity + 1],
        }
    }

    /// Updates the tree by adding a value to the element at a specified index.
    ///
    /// This operation also propagates the update to subsequent elements in the tree.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index where the value should be added.
    /// * `value` - The value to add to the element at the specified index.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success (`Ok`) or an error (`FenwickTreeError::IndexOutOfBounds`)
    /// if the index is out of bounds.
    pub fn update(&mut self, index: usize, value: T) -> Result<(), FenwickTreeError> {
        if index >= self.data.len() - 1 {
            return Err(FenwickTreeError::IndexOutOfBounds);
        }

        let mut idx = index + 1;
        while idx < self.data.len() {
            self.data[idx] += value;
            idx += lowbit(idx);
        }

        Ok(())
    }

    /// Computes the sum of elements from the start of the tree up to a specified index.
    ///
    /// This operation efficiently calculates the prefix sum using the tree structure.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index up to which the sum should be computed.
    ///
    /// # Returns
    ///
    /// A `Result` containing the prefix sum (`Ok(sum)`) or an error (`FenwickTreeError::IndexOutOfBounds`)
    /// if the index is out of bounds.
    pub fn prefix_query(&self, index: usize) -> Result<T, FenwickTreeError> {
        if index >= self.data.len() - 1 {
            return Err(FenwickTreeError::IndexOutOfBounds);
        }

        let mut idx = index + 1;
        let mut result = T::default();
        while idx > 0 {
            result += self.data[idx];
            idx -= lowbit(idx);
        }

        Ok(result)
    }

    /// Computes the sum of elements within a specified range `[left, right]`.
    ///
    /// This operation calculates the range sum by performing two prefix sum queries.
    ///
    /// # Arguments
    ///
    /// * `left` - The zero-based starting index of the range.
    /// * `right` - The zero-based ending index of the range.
    ///
    /// # Returns
    ///
    /// A `Result` containing the range sum (`Ok(sum)`) or an error (`FenwickTreeError::InvalidRange`)
    /// if the left index is greater than the right index or the right index is out of bounds.
    pub fn range_query(&self, left: usize, right: usize) -> Result<T, FenwickTreeError> {
        if left > right || right >= self.data.len() - 1 {
            return Err(FenwickTreeError::InvalidRange);
        }

        let right_query = self.prefix_query(right)?;
        let left_query = if left == 0 {
            T::default()
        } else {
            self.prefix_query(left - 1)?
        };

        Ok(right_query - left_query)
    }

    /// Retrieves the value at a specific index by isolating it from the prefix sum.
    ///
    /// This operation determines the value at `index` by subtracting the prefix sum up to `index - 1`
    /// from the prefix sum up to `index`.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index of the element to retrieve.
    ///
    /// # Returns
    ///
    /// A `Result` containing the value at the specified index (`Ok(value)`) or an error (`FenwickTreeError::IndexOutOfBounds`)
    /// if the index is out of bounds.
    pub fn point_query(&self, index: usize) -> Result<T, FenwickTreeError> {
        if index >= self.data.len() - 1 {
            return Err(FenwickTreeError::IndexOutOfBounds);
        }

        let index_query = self.prefix_query(index)?;
        let prev_query = if index == 0 {
            T::default()
        } else {
            self.prefix_query(index - 1)?
        };

        Ok(index_query - prev_query)
    }

    /// Sets the value at a specific index in the tree, updating the structure accordingly.
    ///
    /// This operation updates the value at `index` by computing the difference between the
    /// desired value and the current value, then applying that difference using `update`.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index of the element to set.
    /// * `value` - The new value to set at the specified index.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success (`Ok`) or an error (`FenwickTreeError::IndexOutOfBounds`)
    /// if the index is out of bounds.
    pub fn set(&mut self, index: usize, value: T) -> Result<(), FenwickTreeError> {
        self.update(index, value - self.point_query(index)?)
    }
}

/// Computes the lowest set bit (rightmost `1` bit) of a number.
///
/// This function isolates the lowest set bit in the binary representation of `x`.
/// It's used to navigate the Fenwick Tree by determining the next index to update or query.
///
///
/// In a Fenwick Tree, operations like updating and querying use bitwise manipulation
/// (via the lowbit function). These operations naturally align with 1-based indexing,
/// making traversal between parent and child nodes more straightforward.
///
/// # Arguments
///
/// * `x` - The input number whose lowest set bit is to be determined.
///
/// # Returns
///
/// The value of the lowest set bit in `x`.
const fn lowbit(x: usize) -> usize {
    x & (!x + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fenwick_tree() {
        let mut fenwick_tree = FenwickTree::with_capacity(10);

        assert_eq!(fenwick_tree.update(0, 5), Ok(()));
        assert_eq!(fenwick_tree.update(1, 3), Ok(()));
        assert_eq!(fenwick_tree.update(2, -2), Ok(()));
        assert_eq!(fenwick_tree.update(3, 6), Ok(()));
        assert_eq!(fenwick_tree.update(4, -4), Ok(()));
        assert_eq!(fenwick_tree.update(5, 7), Ok(()));
        assert_eq!(fenwick_tree.update(6, -1), Ok(()));
        assert_eq!(fenwick_tree.update(7, 2), Ok(()));
        assert_eq!(fenwick_tree.update(8, -3), Ok(()));
        assert_eq!(fenwick_tree.update(9, 4), Ok(()));
        assert_eq!(fenwick_tree.set(3, 10), Ok(()));
        assert_eq!(fenwick_tree.point_query(3), Ok(10));
        assert_eq!(fenwick_tree.set(5, 0), Ok(()));
        assert_eq!(fenwick_tree.point_query(5), Ok(0));
        assert_eq!(
            fenwick_tree.update(10, 11),
            Err(FenwickTreeError::IndexOutOfBounds)
        );
        assert_eq!(
            fenwick_tree.set(10, 11),
            Err(FenwickTreeError::IndexOutOfBounds)
        );

        assert_eq!(fenwick_tree.prefix_query(0), Ok(5));
        assert_eq!(fenwick_tree.prefix_query(1), Ok(8));
        assert_eq!(fenwick_tree.prefix_query(2), Ok(6));
        assert_eq!(fenwick_tree.prefix_query(3), Ok(16));
        assert_eq!(fenwick_tree.prefix_query(4), Ok(12));
        assert_eq!(fenwick_tree.prefix_query(5), Ok(12));
        assert_eq!(fenwick_tree.prefix_query(6), Ok(11));
        assert_eq!(fenwick_tree.prefix_query(7), Ok(13));
        assert_eq!(fenwick_tree.prefix_query(8), Ok(10));
        assert_eq!(fenwick_tree.prefix_query(9), Ok(14));
        assert_eq!(
            fenwick_tree.prefix_query(10),
            Err(FenwickTreeError::IndexOutOfBounds)
        );

        assert_eq!(fenwick_tree.range_query(0, 4), Ok(12));
        assert_eq!(fenwick_tree.range_query(3, 7), Ok(7));
        assert_eq!(fenwick_tree.range_query(2, 5), Ok(4));
        assert_eq!(
            fenwick_tree.range_query(4, 3),
            Err(FenwickTreeError::InvalidRange)
        );
        assert_eq!(
            fenwick_tree.range_query(2, 10),
            Err(FenwickTreeError::InvalidRange)
        );

        assert_eq!(fenwick_tree.point_query(0), Ok(5));
        assert_eq!(fenwick_tree.point_query(4), Ok(-4));
        assert_eq!(fenwick_tree.point_query(9), Ok(4));
        assert_eq!(
            fenwick_tree.point_query(10),
            Err(FenwickTreeError::IndexOutOfBounds)
        );
    }
}
