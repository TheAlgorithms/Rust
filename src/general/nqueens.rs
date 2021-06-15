/* The eight queens' problem is the task of positioning eight chess queens on an 8/8 chessboard so that no two queens threaten each other.
Consequently, no two queens must share the same row, column, or diagonal in order for a solution to be found.
The eight queens problem is a variation on the two queens problem, which entails arranging two non-attacking queens on a board. */

const N : usize = 8;

// Solutions exist for all natural numbers N with the exception of N = 2 and N = 3.
fn t(mut board: &mut [[bool; N]; N], r: usize, mut cnt: &mut i64){
	if r == N {
		*cnt += 1;
		println!("Answer {}\n", *cnt);

		for i in board.iter(){
			println!("{}", i.iter().map(|&x|
			 if x {"Q"} else{"."}.to_string())
			.collect::<Vec<String>>()
			.join(" ")
		    )
		}
	
	println!(" ");
	return;
    }

    for i in 0..N {
	    let mut det: bool = true;
	    for j in 0..r {
		    if board[j][i] || i + j >= r && board[j][i + j - r] || i + r < N + j && board[j][i + r - j] {
			    det  = false;
		    }
	    }

	    if det {
		    board[r][i] = true;
		    t(&mut board, r + 1, &mut cnt);
		    board[r][i] = false;
	    }
    }

}

fn main() {
	let mut board: [[bool; N]; N] = [[false; N]; N];
	let mut cnt: i64 = 0;
	t(&mut board, 0, &mut cnt);
}


/* Sample Output of the following code :- 

Answer 1

Q . . . . . . .
. . . . Q . . .
. . . . . . . Q
. . . . . Q . .
. . Q . . . . .
. . . . . . Q .
. Q . . . . . .
. . . Q . . . .
 
Answer 2

Q . . . . . . .
. . . . . Q . .
. . . . . . . Q
. . Q . . . . .
. . . . . . Q .
. . . Q . . . .
. Q . . . . . .
. . . . Q . . .
 
Answer 3

Q . . . . . . .
. . . . . . Q .
. . . Q . . . .
. . . . . Q . .
. . . . . . . Q
. Q . . . . . .
. . . . Q . . .
. . Q . . . . .
 
Answer 4

Q . . . . . . .
. . . . . . Q .
. . . . Q . . .
. . . . . . . Q
. Q . . . . . .
. . . Q . . . .
. . . . . Q . .
. . Q . . . . .
 
Answer 5

. Q . . . . . .
. . . Q . . . .
. . . . . Q . .
. . . . . . . Q
. . Q . . . . .
Q . . . . . . .
. . . . . . Q .
. . . . Q . . .
 
*/
