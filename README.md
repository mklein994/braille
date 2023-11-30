# braille

Show a graph on the terminal using [block](https://en.wikipedia.org/wiki/Block_Elements) or [braille](https://en.wikipedia.org/wiki/Braille_Patterns) characters.

## Example

### Command

```console
awk -f fancy_graph.awk | braille -r -1:1 -c 5
```

<details>
<summary><code>fancy_graph.awk</code></summary>

```awk
BEGIN {
    pi = atan2(0, -1);
    for (i = (-20 * pi); i < (20 * pi); i++) {
        print 100 * sin(i / 4) / i;
    }
}
```

$$100*\frac{\sin(\frac{x}{4})}{x}$$

</details>

### Output

```plain
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿⣿⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⣿⣿⣿⣿⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣿⣿⣿⣿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⣀⣤⣤⣤⣀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⢠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣇⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⣶⣄⠀⠀⠀⠀⠀⠀⠀⣀⣤⣤⣤⣄⡀
⠉⠉⠉⠉⠉⠉⠉⠻⢿⣿⣿⠿⠋⠉⠉⠉⠉⠉⠉⠹⣿⣿⣿⣿⣿⠋⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⢿⣿⣿⣿⣿⡟⠉⠉⠉⠉⠉⠉⠉⠻⢿⣿⡿⠟⠋⠉⠉⠉⠉⠉⠉
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣿⡿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣿⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
```

### Command

```console
awk -f fancy_graph_small.awk | braille -c 10
```

<details>
<summary><code>fancy_graph_small.awk</code></summary>

```awk
BEGIN {
    pi = atan2(0, -1);
    for (i = -8 * pi; i < 8 * pi; i++) {
        if (i != 0) {
            print sin(i) / i;
        }
    }
}
```
$$\frac{\sin(x)}{x}$$

</details>

### Output

```plain
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⣀⠀⠀⣠⡀⠀⣰⡆⠀⢸⣿⣿⡆⠀⣼⡄⠀⢠⡀⠀⢀⡀⠀⠀
⠹⠟⠉⠹⠟⠉⠹⡿⠉⠙⣿⠉⠉⠉⢹⣿⠉⠹⡿⠉⠙⠿⠉⠉⠻⠃
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⠀⠀⠀⠸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
```

### Command

```jq
# curve.jq
(-1 | acos) as $pi
| range(-8 * $pi; 8 * $pi)
| [
  (. / 5 | cos),
  (. / 4 | sin)
] | @tsv
```

```console
jq -nrf curve.jq | braille \
    --style auto \
    --per 2 \
    --kind braille-columns \
    10
```

### Output

```plain
⠀⠀⣰⣿⡄⠀⠀⠀⠀⠀⠀⠠⠉⠑⣀⣾⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⢠⣿⣿⣿⠀⠀⠀⠀⠀⠠⠁⠀⠀⡸⣿⣿⣇⠀⠀⠀⠀⠀⠀⠀⠀
⠀⣼⣿⣿⣿⡇⠀⠀⠀⠀⠂⠀⠀⢀⠀⢻⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀
⢂⣿⣿⣿⣿⣿⠀⠀⠀⠐⠀⠀⠀⡀⠀⠈⣿⣿⡇⠀⠀⠀⠀⠀⠀⠄
⡘⣿⣿⣿⣿⣿⡆⠀⠀⠂⠀⠀⠀⠀⠀⠀⢹⣿⣿⠀⠀⠀⠀⠀⠠⠀
⠀⢻⣿⣿⣿⣿⣧⠀⠐⠀⠀⠀⠈⠀⠀⠀⠈⣿⣿⡆⠀⠀⠀⠀⠄⠂
⠀⠘⣿⣿⣿⣿⣿⡄⠁⠀⠀⠀⠂⠀⠀⠀⠀⢸⣿⣧⠀⠀⠀⠠⠠⠀
⠀⠀⢻⣿⣿⣿⣿⣏⠀⠀⠀⠐⠀⠀⠀⠀⠀⠀⣿⣿⡄⠀⠀⠄⠄⠀
⠀⠀⠈⣿⣿⣿⡿⠀⠄⠀⠀⠂⠀⠀⠀⠀⠀⠀⠸⣿⣧⠀⡐⠠⠀⠀
⠀⠀⠀⠘⢿⡿⠁⠀⠐⣀⠌⠀⠀⠀⠀⠀⠀⠀⠀⠹⣿⠶⡠⠂⠀⠀
```

## Installation

After [installing `rust`](https://www.rust-lang.org/tools/install):

<!-- TODO: maybe publish to crates.io? -->
```console
cargo install --git https://github.com/mklein994/braille.git
```

## Help (`braille --help`)

~~~plain
Usage: braille [OPTIONS] [SIZE]

Arguments:
  [SIZE]
          How wide or tall the graph can be (defaults to terminal size)

Options:
  -r, --range <[MIN]:[MAX]>
          The input's minimum and maximum values
          
          If provided, at least one of `MIN` or `MAX` must be given.
          
          # Example
          
          ```plain
          --range -3:4  # Use bounds given
          --range -3:   # Automatically determine maximum
          --range :4    # Automatically determine minimum
          ```

  -p, --per <PER>
          Number of values per line of input
          
          When the graph kind supports it, each value represents a separate series.
          
          [default: 1]

  -s, --style <STYLE>
          How the space between multiple series should be handled
          
          [default: filled]

          Possible values:
          - auto:   Fill space between series when the first is less or equal to the second value, hollow otherwise
          - line:   Never fill between series (show each independently as a line)
          - filled: Always fill the space between multiple series

  -m, --modeline
          Interpret arguments from the very first line of the input
          
          If this is passed, then the first line from standard input should match the following:
          
          ```plain
          braille [OPTIONS] [ARGUMENTS]
          [VALUES...]
          ```
          
          Where `OPTIONS` and `ARGUMENTS` are space separated values as you would pass them on the
          command line, and `VALUES` are the values you want to graph.
          
          # Example
          
          `input.txt`
          
          ```plain
          braille -r -3:4 4
          -3
          -2
          -1
          0
          1
          2
          3
          4
          ```
          
          ## Command
          
          ```console
          cat input.txt | braille --modeline
          ```
          
          ## Output
          
          ```plain
          ⠙⢿
          ⠀⢸⣷⣄
          ```

  -k, --kind <GRAPH_KIND>
          The kind of graph to print
          
          Kinds supported with their matching option parameters:
          
          | Kind    | Bar (horizontal) | Column (vertical)      |
          |---------|------------------|------------------------|
          | Braille | `braille` (-b)   | `braille-columns` (-c) |
          | Block   | `bars` (-B)      | `columns` (-C)         |
          
          [default: braille]

          Possible values:
          - bars:            █▉▊▋▌▍▎▏ Bar graph with block characters
          - columns:         ▁▂▃▄▅▆▇█ Column graph with block characters
          - braille:         ⠙⣇ Bar graph with braille characters
          - braille-columns: ⡶⠚ Column graph with braille characters

  -B
          Shortcut for --kind bars

  -C
          Shortcut for --kind columns

  -b
          Shortcut for --kind braille

  -c
          Shortcut for --kind braille-columns

  -f, --file <FILE>
          Path to file to read from (defaults to standard input)

      --use-full-default-height
          Use the full height if none given
          
          By default, space is given for the prompt (either at the terminal or through a pager like `less`). Use this flag to instead take up the full height given. Passing a size overrides this flag. Does nothing if the graph is not vertical.
          
          [env: BRAILLE_USE_FULL_DEFAULT_HEIGHT=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
~~~
