CRG = cargo
source = src/main.rs

.PHONY: clean all build_new

all: $(source)
		$(CRG) build --release --target x86_64-pc-windows-msvc
		mv ./target/x86_64-pc-windows-msvc/release/new_rust_project.exe gb_checker.exe
		echo shortcut.vbs gb_checker.exe
		echo exit 0
		cmd 
		
clean:
		$(CRG) clean
		rm gb_checker.exe
		
build_new: clean all