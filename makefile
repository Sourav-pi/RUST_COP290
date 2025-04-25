run:
	cd cli
	cargo build --release -p cli
	cd ..
	# rename cli to spreadsheet
	mv target/release/cli target/release/spreadsheet

docs:
	cargo doc --document-private-items --no-deps

coverage:
	cargo tarpaulin --workspace -e gui

test:
	cargo test

ext1 :
	cargo run --release -p gui

clean:
	cargo clean

