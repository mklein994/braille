# curve-1.jq
(-1 | acos) as $pi
| range(-8 * $pi; 8 * $pi)
| . / 5
| cos
