zoo-scanner
=====
A simple scanner by Rust. It can scan only simple XSS, directory traversal, 
and OS command injection. Also it can check if target paths exist.

## Warning

Do not abuse this tool. 
This tool is not intended to encourage malicious behavior. 
It should be used for learning purposes only. 
Any development, production or operation of it must be done at your own risk and discretion. 

## Usage

Clone this repository:

```sh
$ git clone https://github.com/jinnzoo/zoo-scanner.git
```

Build it in your crate root:

```sh
$ cd zoo-scanner
$ cargo build
```

Config search target paths to conf/conf.toml:
```toml
[pathcheck]
targets = ["to/wordpress/login.php", "login.php"]
```

Scan!
```sh
$ mv conf/conf.toml target/debug/
$ cd target/debug/
$ ./zoo-scanner -t http://localhost:8888 -x -d -o
```

