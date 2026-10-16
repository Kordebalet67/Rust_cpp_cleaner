Simple utilita written to clear directory from unnecessary for me files from c++ compiling. I'm interested only in cpp files, so "obj", "ilk", "pdb", "exe" files must be removed in directory Trash.

To build script run cargo build --release in cmd or powershell

To start exe file run .\Rust_cpp_cleaner.exe <absolute path, you want to clean> <file extensions you wwant to remove written throug space>
EX: .\Rust_cpp_cleaner.exe D:\Programming\c++ obj ilk pdb exe
