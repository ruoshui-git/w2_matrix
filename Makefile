all:
	cargo test -- --nocapture --test-threads=1

clean:
	cargo clean