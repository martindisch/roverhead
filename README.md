# roverhead

A small example & benchmark to try out extendr.

## Build

```console
$ R CMD build .
$ R CMD INSTALL --build --library /tmp/trash roverhead_0.0.0.9000.tar.gz
```

## Install

```console
$ R_LIBS_USER=lib R CMD INSTALL roverhead_0.0.0.9000_R_x86_64-pc-linux-gnu.tar.gz
# or
$ R_LIBS_USER=lib R
> install.packages("roverhead_0.0.0.9000_R_x86_64-pc-linux-gnu.tar.gz")
```

## License

[MIT License](LICENSE)
