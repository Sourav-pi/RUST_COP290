run:
	cd cli
	cargo build --release -p cli
	cd ..
	# rename cli to spreadsheet
	mv target/release/cli target/release/spreadsheet

clean:
	cargo clean