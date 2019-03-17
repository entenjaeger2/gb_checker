CRG = cargo
source = src/main.rs

.PHONY: clean all build_new

build: $(source)
		$(CRG) build --release --target x86_64-pc-windows-msvc
		mv ./target/x86_64-pc-windows-msvc/release/gb-checker.exe gb_checker.exe

startup:
		cmd /c shortcut.vbs gb_checker.exe
	
all: build startup

clean:
		rm gb_checker.exe
		#$(CRG) clean
		
build_new: clean all