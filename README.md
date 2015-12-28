# VObject parser and generator for Rust

[![Build Status](https://travis-ci.org/untitaker/rust-vobject.svg?branch=master)](https://travis-ci.org/untitaker/rust-vobject)

This is a parser and generator for the
[vCard](https://tools.ietf.org/html/rfc6350) and
[iCalendar](https://tools.ietf.org/html/rfc5545) formats.

- It doesn't follow all relevant RFCs, and some fundamental things such as
  property encodings are missing.
- Consequently, the API is still unstable, and it's hard to predict how it
  would have to change if new RFCs are implemented.

Nevertheless, I use it for [a simple addressbook
script](https://github.com/untitaker/mates.rs), and you're encouraged to test
it and give feedback in the issue tracker.

Sourcecode is available on [GitHub](https://github.com/untitaker/rust-vobject).
The API documentation is [online
available](http://rust-vobject.unterwaditzer.net), or you can build it
yourself:

    make docs

The testsuite can be run with:

    make test


It is licensed under MIT, see `LICENSE`.
