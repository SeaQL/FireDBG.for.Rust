## Troubleshooting

### macOS

- linking with `cc` failed; No developer tools were found
	```
	error: linking with `cc` failed: exit status: 1
		|
		= note: xcode-select: note: No developer tools were found, requesting install.
	```

	Try: Open terminal and execute command to install `Xcode`
	```shell
	xcode-select --install
	```

### Linux / Windows (WSL 2)

- linker `cc` not found
	```
	error: linker `cc` not found
		|
		= note: No such file or directory (os error 2)
	```

	Try: Open terminal and execute command to install `build-essential`
	```shell
	sudo apt install build-essential
	```

- shared library `libunwind.so` not found
	```
	error while loading shared libraries: libunwind.so.1
	```

	Try: Open terminal and execute command to install `libunwind`
	```shell
	sudo apt install libunwind-15
	```

- shared library `libc++abi.so` not found
	```
	error while loading shared libraries: libc++abi.so.1
	```

	Try: Open terminal and execute command to install `libc++abi`
	```shell
	ubuntu-22.04$ sudo apt install libc++abi1-14
	ubuntu-20.04$ sudo apt install libc++abi1-10
	```

- Link c++ to `clang` instead of g++
	```
	c++: error: unrecognized command-line option '-stdlib=libc++'
	```

	Try: Open terminal and execute command to link c++ to `clang`
	```shell
	sudo update-alternatives --config c++
	```
