# Experiments with Config File formats

# So many to choose from

* yaml
  * how hard is it for human editing???
  * [x] try serde_any to see what a struct looks like in yaml
* strict yaml
* json
* jsonc (json with comments)
  * invented by vscode, but they went with json5
* ron
* hjson
  * doesn't really have full serde serialize support... :/
* json5
* toml

# Requirements

* is popular
* supported by rust serde
* editing supported by emacs (my editor of choice)
* easy to edit by hand
* supports comments

# Language Support

## Rust

Look at the "config" crate, maybe that's a good fit?

## C++

TODO

flatbuffers provides a nice way to serialize datastructures to json,
but it's not pure json.

Here's an idea to serialize structures to yaml from c++:
http://jrruethe.github.io/blog/2015/08/17/yaml-de-slash-serialization-with-boost-fusion/

There's gotta be other libs that do something like this...

# Scoring

|          | yaml | json | jsonc | ron | hjson | json5 | toml | strict yaml | jsonnet | hocon |
|----------|------|------|-------|-----|-------|-------|------|-------------|---------|-------|
| popular  | x    | x    | -     | -   | -     | -     | x    | -           | .5      | -     |
| c++      |      |      |       |     |       |       |      |             |         |       |
| serde    | x    | x    | x     | x   | -     | x     | x    | x           | -       | x     |
| emacs    | x    | x    | -     | x   | x     | -     | x    | x           | x       | -     |
| editable | .5   | -    | .5    | -   | x     | x     | x    | x           | .5      | .5    |
| comments | x    | -    | x     | x   | x     | x     | x    | x           | x       | x     |
|          |      |      |       |     |       |       |      |             |         |       |
| Score    | 4.5  | 3    | 2.5   | 3   | 4     | 3     | 5    | 4           | 3       | 2.5   |

# Results

Get a better idea on the formats looking at the same data in multiple formats.

Wrote a rust serde example to see generated structures for both

## toml

Good for simple structures, better than ini, json, yaml

## yaml

Pretty good for simple, but way better than toml for nested
structures. strictyaml is even nicer...

