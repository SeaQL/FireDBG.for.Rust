## Troubleshooting

### Linux / Windows (WSL 2)

> Supported Linux distributions: Ubuntu 22.04, Ubuntu 20.04, Debian 12, Fedora 39

> Supported Windows (WSL 2) distributions: Ubuntu 22.04, Ubuntu 20.04

- linker `cc` not found
	```
	error: linker `cc` not found
		|
		= note: No such file or directory (os error 2)
	```

	Try: Open terminal and execute command to install `build-essential`
	```shell
	# Ubuntu
	sudo apt install build-essential
	
	# Debian
	sudo apt install build-essential
	
	# Fedora
	sudo dnf groupinstall "Development Tools"
	
	# CentOS
	sudo yum groupinstall "Development Tools"
	```

- shared library `libc++abi.so` not found
	```
	error while loading shared libraries: libc++abi.so.1
	```

	Try: Open terminal and execute command to install `libc++abi`
	```shell
	# Ubuntu 22.04
	sudo apt install libc++abi1-15
	
	# Ubuntu 20.04
	sudo apt install libc++abi1-10
	
	# Debian 12
	sudo apt install libc++abi1-14
	
	# Debian 10
	sudo apt install libc++abi1-7

	# Fedora 39
	sudo dnf install libcxxabi

	# CentOS Stream 9
	curl -sSfL https://github.com/SeaQL/FireDBG.for.Rust/releases/download/1.74.0/libcxx-centos9.tar.gz -o libcxx-centos9.tar.gz
	tar -xvf libcxx-centos9.tar.gz -C /
	```

- shared library `libunwind.so` not found
	```
	error while loading shared libraries: libunwind.so.1
	```

	Try: Open terminal and execute command to install `libunwind`
	```shell
	sudo apt install libunwind-15
	```

- Link c++ to `clang` instead of g++
	```
	c++: error: unrecognized command-line option '-stdlib=libc++'
	```

	Try: Open terminal and execute command to link c++ to `clang`
	```shell
	sudo update-alternatives --config c++
	```

### macOS

> Supported macOS versions: macOS 13 (Ventura), macOS 14 (Sonoma)

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
