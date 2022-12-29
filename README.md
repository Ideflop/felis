# felis

Felis is a command line tool written in Rust that allows you to search the internet from your terminal using w3m.

## Installation

To install felis, you will need to have Rust and w3m installed on your system. You can use the provided ``` installer.sh ``` script to automatically install the dependencies and compile felis but you need to have Rust installed, or you can follow these steps manually:

1. Install Rust by following the instructions at https://www.rust-lang.org/tools/install.

2. Install w3m by following the instructions for your specific operating system. On a Debian-based system, you can use ```apt-get install w3m```.

3. Clone the felis repository and navigate to the project directory:
```
git clone https://github.com/Ideflop/felis.git    
cd felis
```

4.  Compile felis using ```cargo build --release```.

5.  Move the compiled binary to a directory in your ```PATH```(such as ```/usr/local/bin``` or ```/usr/bin```).


## Usage

To search the internet using ```felis```, simply run the felis command followed by your search query:
```
felis how to install rust
```

You can also use the -a flag to create an alias:
```
felis -a
```
Then you can run felis by running the alias

If you want to open a specific URL, you can use the ```-u``` flag:
```
felis -u https://www.rust-lang.org/
```

## Know Issue

if the ```felis``` directory exist in ```.config``` and there is no config file in it, then there will be an error. To fix it remove the ```felis``` directory in the ```.config``` directory.
