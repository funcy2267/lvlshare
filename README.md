# lvlshare ![alt text](https://github.com/funcy2267/lvlshare/blob/master/icon.png "lvlshare")
Export and import Geometry Dash levels as files!

# How it works
There are 2 buttons: Export level, import file. 
The first one converts a level to a .lvl file, which can be shared like any other file,
the other imports such a file and pushes it to the top of your levels list inside Geometry Dash.

For convenience, the program reads your Geometry Dash save file for all level names and puts them
in a selectable list, so you don't have to write it manually.

# Usage
+ Go to releases, and download the latest .zip file.
+ Unzip it somewhere
+ Run the file called `lvlshare.exe`

# Building from source
### Requirements
- [Rust](https://www.rust-lang.org/tools/install)
- [Build tools for Visual Studio 2022](https://visualstudio.microsoft.com/pl/downloads/#build-tools-for-visual-studio-2022) (install C++ build tools)

Clone and enter the repository, then run the program with (you may need to place `sciter.dll` from releases in the same folder as binary):
```
cargo run
```
