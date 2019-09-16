pig-latin
-------

**pig-latin** is cli tool for transcripting english into pig-latin (and reverse in the future). Inspired by one of programming tasks from amazing book [The Rust Programming Language](https://doc.rust-lang.org/stable/book/title-page.html).

Building
------------

**pig-latin** tool is completely written in rust with no dependencies at all. Execute below commands to build project on your unix machine:

    $ git clone https://github.com/thinkofher/unixify.git
    $ cd pig-latin
    $ cargo -q build --release

Sample Usage
----

    $ echo "my name is foobar" | ./target/release/pig-latin
    ymay amenay ishay oobarfay
