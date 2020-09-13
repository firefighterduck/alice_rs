CRG = cargo
source = src/*.*

.PHONY: all buil clean

build_win: $(source)
		$(CRG) build --release --target x86_64-pc-windows-msvc
		mv ./target/x86_64-pc-windows-msvc/release/alice_rs.exe alice.exe

build_nix: $(source)
		$(CRG) build --release --target x86_64-unknown-linux-gnu
		mv ./target/x86_64-unknown-linux-gnu/release/alice_rs alice

clean:
		rm alice*
		cargo clean
		
all: clean build