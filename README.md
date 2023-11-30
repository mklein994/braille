# braille

Show a graph on the terminal using [block](https://en.wikipedia.org/wiki/Block_Elements) or [braille](https://en.wikipedia.org/wiki/Braille_Patterns) characters.

## Example

### Fancy Graph

#### Command

```console
awk -f fancy_graph.awk | braille -r -1:1 -c 5
```

```awk
# fancy_graph.awk
BEGIN {
    pi = atan2(0, -1);
    for (i = (-20 * pi); i < (20 * pi); i++) {
        print 100 * sin(i / 4) / i;
    }
}
```

$$100*\frac{\sin(\frac{x}{4})}{x}$$

#### Output

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

### Fancy Graph (Small)

#### Command

```console
awk -f fancy_graph_small.awk | braille -c 10
```

```awk
# fancy_graph_small.awk
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

#### Output

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

### Curve

#### Command

```console
jq -nrf curve.jq | braille \
    --style auto \
    --per 2 \
    --kind braille-columns \
    10
```

```jq
# curve.jq
(-1 | acos) as $pi
| range(-8 * $pi; 8 * $pi)
| [
  (. / 5 | cos),
  (. / 4 | sin)
] | @tsv
```

#### Output

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

For help on the various options, see `braille --help` or run `cargo doc` on the local clone and browse the API docs.
