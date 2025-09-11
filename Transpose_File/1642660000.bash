# Read from the file file.txt and print its transposed content to stdout.
awk '
{
  for (i = 1; i <= NF; i++) {
    a[i, NR] = $i
  }
  max_cols = NF
  max_rows = NR
}
END {
  for (i = 1; i <= max_cols; i++) {
    for (j = 1; j <= max_rows; j++) {
      printf "%s%s", a[i, j], (j == max_rows ? "\n" : " ")
    }
  }
}' file.txt
