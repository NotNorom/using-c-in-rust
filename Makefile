build:
	cd ./is-c && make
	cd ./uses-c && cargo build

run: build
	cd ./uses-c && cargo run

clean:
	$(RM) ./is-c/out/*
	cd uses-c && cargo clean && cd ../