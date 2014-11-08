# VObject parser for Rust

[![Build Status](https://travis-ci.org/untitaker/rust-vobject.svg?branch=master)](https://travis-ci.org/untitaker/rust-vobject)

This is a primitive unstable vobject parser for Rust, originally written for [a
simple addressbook script](https://github.com/untitaker/mates.rs) and based on
[rust-peg](https://github.com/kevinmehall/rust-peg).

The author hasn't read the relevant RFCs, so this library handles all versions
of VCard and iCalendar equally badly. The most important missing feature is
that this library doesn't know how to do value escaping.

See the testsuite for some basic usage. This is an experiment by me to learn
Rust, and the API is too imperfect to be documented. Suggestions are welcome.
