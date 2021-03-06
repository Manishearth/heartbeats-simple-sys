# Simple Heartbeats

This library provides simplified performance, accuracy, and power monitoring.

See
[https://github.com/libheartbeats/heartbeats](https://github.com/libheartbeats/heartbeats)
for the original libraries that support shared memory and manage memory and
energy readers automatically.

## Building

This project uses CMake.

To build all libraries, run:

``` sh
mkdir _build
cd _build
cmake ..
make
```

## Installing

To install all libraries and headers, run with proper privileges:

``` sh
make install
```

On Linux, installation typically places libraries in `/usr/local/lib` and
header files in `/usr/local/include/heartbeats-simple`.

## Uninstalling

Install must be run before uninstalling in order to have a manifest.

To remove libraries and headers installed to the system, run with proper
privileges:

``` sh
make uninstall
```

## Cleaning

To cleanup build artifacts, run

``` sh
make clean
```
