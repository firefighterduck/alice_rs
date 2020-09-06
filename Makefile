CRG = cargo
source = src/main.rs

.PHONY: clean all build_new

build: $(source)
		$(CRG) build --release
		mv ./target/release/alice_rs.exe alice.exe

clean:
		rm alice.exe
		cargo clean
		
build_new: clean build