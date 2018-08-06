##What is CHIP-8?##
CHIP-8 is an interpreted programming language, developed by Joseph Weisbecker. It was initially used on the COSMAC VIP and Telmac 1800 8-bit microcomputers in the mid-1970s. CHIP-8 programs are run on a CHIP-8 virtual machine. It was made to allow video games to be more easily programmed for these computers. 

For more information visit https://en.wikipedia.org/wiki/CHIP-8

##How to run##
On linux, install cargo and clone the repo.
```
git clone https://github.com/jajajasalu2/CHIP-8
sudo apt update
sudo apt install libsdl2-dev
sudo apt install cargo
```
An example to run a game:
```
cargo run pong2.c8
```
