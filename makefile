REPORT = ./report.tex
compile_cli:
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

report:
	@ pdflatex -quiet $(REPORT) 

clean:
	cargo clean

.PHONY: all compile_cli docs coverage test ext1 report clean
