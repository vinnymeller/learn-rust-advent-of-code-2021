iter:
	mv src/main_iter.rs src/main.rs
	time for i in {1..10}; do cargo run -- real.txt &> /dev/null; done
	mv src/main.rs src/main_iter.rs

vec:
	mv src/main_vec.rs src/main.rs
	time for i in {1..10}; do cargo run -- real.txt &> /dev/null; done
	mv src/main.rs src/main_vec.rs
