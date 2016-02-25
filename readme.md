# rustpwhash

sometimes you've to generate a bunch of salts and bcrypt hashes.

for example you can pipe sql (line by line) to rustpwhash,
rustpwhash will look for "pw:[PW]" at the end of the string.

example:

```
INSERT INTO users (pw, salt) VALUES {pw} {salt}  pw:abc1
```

outputs:

```
INSERT INTO users (pw, salt) VALUES $2y$07$Ww7o4t7xxFs3p5mM9qu59eXIqm.EWywd7QTOWySkD38TaqSipdsly Ww7o4t7xxFs3p5mM9qu59n
```

for every line in the input stream rustpwhash will generate a bcrypt hash and and the hashed pw.

# Installation

```
git clone https://github.com/timglabisch/rust_bcrypt_stdin.git
cd rust_bcrypt_stdin
cargo build --release
```

Usage:

Arguments:

rustpwhash [THREADS] [COST]

Threads should be [CPU CORES] x 2

Example:

```
cat rows.txt | ./target/release/rustpwhash 4 7
```

for easy testing the rows.txt is part of the repository.
