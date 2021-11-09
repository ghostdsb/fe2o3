/**
*

   Instructions

   Tests

   Results
   Instructions
   Detect saddle points in a matrix.

   So say you have a matrix like so:

       1  2  3
   |---------
   1 | 9  8  7
   2 | 5  3  2     <--- saddle point at column 1, row 2, with value 5
   3 | 6  6  7
   It has a saddle point at column 1, row 2.

   It's called a "saddle point" because it is greater than or equal to every element in its row and less than or equal to every element in its column.

   A matrix may have zero or more saddle points.

   Your code should be able to provide the (possibly empty) list of all the saddle points for any given matrix.

   The matrix can have a different number of rows and columns (Non square).

   Note that you may find other definitions of matrix saddle points online, but the tests for this exercise follow the above unambiguous definition.

*/

#[allow(unused)]
pub mod saddle_points {
  pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddles = vec![];
    for i in 0..input.len() {
      let row = &input[i];
      for j in 0..row.len() {
        if max_in_row(&input[i][j], i, input) && min_in_col(&input[i][j], j, input) {
          saddles.push((i, j))
        }
      }
    }
    return saddles;
  }
  fn max_in_row(val: &u64, row: usize, matrix: &[Vec<u64>]) -> bool {
    match matrix[row].iter().max() {
      Some(max_val) => max_val == val,
      None => false,
    }
  }
  fn min_in_col(val: &u64, col: usize, matrix: &[Vec<u64>]) -> bool {
    let mut min = val;
    for i in 0..matrix.len() {
      if matrix[i][col] < *val {
        min = &matrix[i][col];
      }
    }
    min == val
  }
}
