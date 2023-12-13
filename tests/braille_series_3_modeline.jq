"braille -c -sl -p3 10", ((-1 | acos) as $pi | range(-8 * $pi; 8 * $pi) | [(. / 5 | cos), (. / 4 | sin), (. / 2 | sin)] | @tsv)
