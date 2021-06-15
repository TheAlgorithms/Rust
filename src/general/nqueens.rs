const N : usize = 8;


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
