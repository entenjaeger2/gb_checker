CRG = cargo
source = src/main.rs

.PHONY: clean all build_new

build: $(source)
		$(CRG) build --release --target x86_64-pc-windows-msvc
		mv ./target/x86_64-pc-windows-msvc/release/gb-checker.exe gb_checker.exe

build_feature_dynamic: $(source)
		$(CRG) build --release --target x86_64-pc-windows-msvc --features "dynamic-data"
		mv ./target/x86_64-pc-windows-msvc/release/gb-checker.exe gb_checker.exe

startup:
		cmd /c shortcut.vbs gb_checker.exe
	
all: build startup

clean:
		rm -f gb_checker.*
		
build_new: clean all

build_new_dyn: clean build_feature_dynamic