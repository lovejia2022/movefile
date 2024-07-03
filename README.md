MOVEFILE(1)

# NAME

```
movefile - move (rename) files
```

# SYNOPSIS

```
movefile [OPTIONS] SRC DST
movefile [OPTIONS] -i|--into SRC... DST
```

# OPTIONS

- -c,--copy      Copy instead of move.
- -o,--override  Override target file if target file is exits.
- -m,--merge     Merge source directory into target directory.

# RETURN CODE

- 1     Usage error.
- 2     Target exists.
- 100   Internal error.

# Links

- Source: https://github.com/lovejia2022/movefile
