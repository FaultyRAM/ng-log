# ng-log

[![Build status][1]][2]

`ng-log` is a library which handles ngLog processing and transmission.

## About ngLog, ngStats and ngWorldStats

ngLog, ngStats and ngWorldStats were used by video games such as *Unreal
Tournament* for recording and collecting player statistics. Games using these
technologies did so via the following series of procedures:

1. Write to the filesystem an ngLog-formatted file describing a series of
   events.
2. Invoke the *local batcher*, which processes the log file, updates the *local
   database* containing statistics for the current server, and generates an
   HTML representation of said statistics.
3. If the server is hosting a network (i.e. not single-player) game, also
   invoke the *world batcher*, which transmits the statistics from the log file
   to a remote server. This server updates its own *world database* and
   generates an HTML representation of all the statistics it has collected.

## Usage

To use the `ng-log` library in a [Cargo][3]-based project, add the following to
the project's `Cargo.toml`:

    [dependencies]
    ng-log = "0.1.0"

And add the following to the project's crate root:

    extern crate ng_log;

[1]: https://travis-ci.org/FaultyRAM/ng-log.svg?branch=master
[2]: https://travis-ci.org/FaultyRAM/ng-log
[3]: https://crates.io
