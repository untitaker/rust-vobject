# VObject parser for Rust

**This is an experiment by me to learn Rust, performance and API are bad.**

[![Build Status](https://travis-ci.org/untitaker/rust-vobject.svg?branch=master)](https://travis-ci.org/untitaker/rust-vobject)

This is a primitive VObject and iCalendar parser for Rust, originally written
for [a simple addressbook script](https://github.com/untitaker/mates.rs) and
based on [rust-peg](https://github.com/kevinmehall/rust-peg).

Sourcecode is available on [GitHub](https://github.com/untitaker/rust-vobject).
The API documentation is [online
available](http://rust-vobject.unterwaditzer.net), or you can build it
yourself:

    make docs

The testsuite can be run with:

    make test


It is licensed under MIT, see `LICENSE`.
