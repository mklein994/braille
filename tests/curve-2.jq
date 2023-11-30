(-1 | acos) as $pi
| range(-8 * $pi; 8 * $pi)
| [
  (. / 5 | cos),
  (. / 4 | sin)
] | @tsv
