jsonfmt
=======

[![Build Status](https://github.com/anderejd/jsonfmt/workflows/Rust/badge.svg)](https://github.com/anderejd/jsonfmt/actions)

A tiny command line program for pretty printing/formatting of json files.

Usage examples
--------------

Prettify and overwrite an existing file:
```
jsonfmt some_file.json
```

Read json from stdin, prettify it and print to stdout:
```
cat in_file.json | jsonfmt
```

Read json from stdin, minimize it and write it to a new file:
```
cat in_file.json | jsonfmt -m -o out_file.json
```

Changelog
---------

### 0.3.0
 - BUGFIX: Preserve order of object properties.
 - BUGFIX: Preserve original number precision.
 - Feature: `-m --minimize` feature.
 - Feature: `-o` for writing to a new file.
 - Feature: Reading from stdin.
 - Feature: Writing to stdout.

### 0.2.0
 - Traded some memory usage for more speed.

### 0.1.1
 - Added a readme.

### 0.1.0
 - A first tiny experiment, but working.

