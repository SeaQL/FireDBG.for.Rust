#!/bin/bash

# https://www.gnu.org/software/bash/manual/bash.html#The-Set-Builtin
set -u

get_firedbg_version() {
    local _rustc_version="$(rustc --version)"

    case "$_rustc_version" in
        rustc\ 1.74.*)
            local _firedbg_version="1.74.2"
            ;;
        rustc\ 1.75.*)
            local _firedbg_version="1.75.1"
            ;;
        rustc\ 1.76.*)
            local _firedbg_version="1.76.0"
            ;;
        rustc\ 1.77.*)
            local _firedbg_version="1.77.1"
            ;;
        rustc\ 1.78.*)
            local _firedbg_version="1.78.0"
            ;;
        rustc\ 1.79.*)
            local _firedbg_version="1.79.0"
            ;;
        *)
            err "no precompiled binaries available for $_rustc_version";
            ;;
    esac

    RETVAL="$_firedbg_version"
}

main() {
    downloader --check
    need_cmd uname
    need_cmd mktemp
    need_cmd mkdir
    need_cmd rm
    need_cmd tar
    need_cmd which

    get_architecture || return 1
    local _arch="$RETVAL"
    assert_nz "$_arch" "arch"

    which rustup > /dev/null 2>&1
    need_ok "failed to find Rust installation, is rustup installed?"

    get_firedbg_version || return 1
    local _firedbg_version="$RETVAL"
    assert_nz "$_firedbg_version" "firedbg version"

    local _url="https://github.com/SeaQL/FireDBG.for.Rust/releases/download/$_firedbg_version/$_arch.tar.gz"
    local _dir="$(mktemp -d 2>/dev/null || ensure mktemp -d -t FireDBG)"
    local _file="$_dir/$_arch.tar.gz"

    set +u
    local _cargo_home="$CARGO_HOME"
    if [ -z "$_cargo_home" ]; then
        _cargo_home="$HOME/.cargo";
    fi
    local _cargo_bin="$_cargo_home/bin"
    ensure mkdir -p "$_cargo_bin"
    set -u

    printf '%s `%s`\n' 'info: downloading FireDBG from' "$_url" 1>&2

    ensure mkdir -p "$_dir"
    downloader "$_url" "$_file"
    if [ $? != 0 ]; then
        say "failed to download $_url"
        say "this may be a standard network error, but it may also indicate"
        say "that FireDBG's release process is not working. When in doubt"
        say "please feel free to open an issue!"
        exit 1
    fi
    ensure tar xf "$_file" --strip-components 1 -C "$_dir"

    printf '%s `%s`\n' 'info: installing FireDBG binaries to' "$_cargo_bin" 1>&2

    ignore rm -rf "$_cargo_bin/firedbg*"
    ignore rm -rf "$_cargo_bin/firedbg-lib"

    ensure mv "$_dir/firedbg-lib" "$_cargo_bin/firedbg-lib"
    ensure mv "$_dir/firedbg" "$_cargo_bin/firedbg"
    ensure mv "$_dir/firedbg-indexer" "$_cargo_bin/firedbg-indexer"
    ensure mv "$_dir/firedbg-debugger" "$_cargo_bin/firedbg-debugger"

    printf '%s\n' 'info: performing FireDBG self tests' 1>&2

    local _self_test="$_cargo_home/bin/firedbg-lib/debugger-self-test"

    cd "$_self_test"
    "$_cargo_home/bin/firedbg" run debugger_self_test --output "$_self_test/output.firedbg.ss"
    cd - > /dev/null

    if [ $? != 0 ]; then
        say "fail to run FireDBG debugger"
        exit 1
    fi

    "$_cargo_home/bin/firedbg-indexer" --input "$_self_test/output.firedbg.ss" validate --json "$_self_test/expected_data.json"

    if [ $? != 0 ]; then
        say "fail to validate FireDBG debugger result"
        exit 1
    fi

    printf '%s\n' 'info: completed FireDBG self tests' 1>&2
}

get_architecture() {
    local _ostype="$(uname -s)"
    local _cputype="$(uname -m)"

    set +u
    if [ -n "$TARGETOS" ]; then
        _ostype="$TARGETOS"
    fi

    if [ -n "$TARGETARCH" ]; then
        _cputype="$TARGETARCH"
    fi
    set -u

    if [ "$_ostype" = Darwin ] && [ "$_cputype" = i386 ]; then
        if sysctl hw.optional.x86_64 | grep -q ': 1'; then
            local _cputype=x86_64
        fi
    fi

    case "$_ostype" in
        Linux | linux)
            local _os_id="$(awk -F= '$1=="ID" { print $2 ;}' /etc/os-release | tr -d '"')"
            local _os_version_id="$(awk -F= '$1=="VERSION_ID" { print $2 ;}' /etc/os-release | tr -d '"')"
            local _ostype="$_os_id$_os_version_id"
            case "$_ostype" in
                pop*)
                    local _ostype="ubuntu$_os_version_id"
                    ;;
            esac
            local _os_id_like="$(awk -F= '$1=="ID_LIKE" { print $2 ;}' /etc/os-release | tr -d '"')"
            case "$_os_id" in
                linuxmint*)
                    case "$_os_id_like" in
                        ubuntu*)
                        case "$_os_version_id" in
                            21*)
                                local _ostype="ubuntu22"
                                ;;
                            20*)
                                local _ostype="ubuntu20"
                                ;;
                        debian*)
                            local _ostype="debian12"
                            ;;
                    esac
            esac                    
            case "$_ostype" in
                ubuntu22*)
                    check_apt_install libc++abi1-15
                    ;;
                ubuntu20*)
                    check_apt_install libc++abi1-10
                    ;;
                debian12*)
                    check_apt_install libc++abi1-14
                    ;;
                debian10*)
                    check_apt_install libc++abi1-7
                    ;;
                fedora39*)
                    check_dnf_install libcxxabi
                    ;;
                centos9*)
                    check_yum_install_rpm libcxxabi https://kojipkgs.fedoraproject.org//packages/libcxx/17.0.4/1.fc39/x86_64/libcxxabi-17.0.4-1.fc39.x86_64.rpm
                    ;;
                arch* | manjaro* | endeavouros* | garuda*)
                    check_pacman_install libc++abi
                    local _ostype="ubuntu20.04"
                    ;;
                *)
                    err "no precompiled binaries available for OS: $_ostype"
                    ;;
            esac
            ;;
        Darwin)
            local _ostype=darwin
            ;;
        MINGW* | MSYS* | CYGWIN*)
            err "please run this installation script inside Windows Subsystem for Linux (WSL 2)"
            ;;
        *)
            err "no precompiled binaries available for OS: $_ostype"
            ;;
    esac

    case "$_cputype" in
        x86_64 | x86-64 | x64 | amd64)
            local _cputype=x86_64
            ;;
        arm64 | aarch64)
            local _cputype=aarch64
            ;;
        *)
            err "no precompiled binaries available for CPU architecture: $_cputype"
    esac

    if [ "$_cputype" = aarch64 ] && [ "$_ostype" = apple-darwin ]; then
        _cputype="x86_64"
    fi

    local _arch="$_cputype-$_ostype"

    RETVAL="$_arch"
}

say() {
    echo "FireDBG: $1"
}

err() {
    say "$1" >&2
    exit 1
}

need_cmd() {
    if ! check_cmd "$1"
    then err "need '$1' (command not found)"
    fi
}

check_cmd() {
    command -v "$1" > /dev/null 2>&1
    return $?
}

need_ok() {
    if [ $? != 0 ]; then err "$1"; fi
}

assert_nz() {
    if [ -z "$1" ]; then err "assert_nz $2"; fi
}

# Run a command that should never fail. If the command fails execution
# will immediately terminate with an error showing the failing
# command.
ensure() {
    "$@"
    need_ok "command failed: $*"
}

ignore() {
    "$@"
}

# This wraps curl or wget. Try curl first, if not installed,
# use wget instead.
downloader() {
    if check_cmd curl
    then _dld=curl
    elif check_cmd wget
    then _dld=wget
    else _dld='curl or wget' # to be used in error message of need_cmd
    fi

    if [ "$1" = --check ]
    then need_cmd "$_dld"
    elif [ "$_dld" = curl ]
    then curl -sSfL "$1" -o "$2"
    elif [ "$_dld" = wget ]
    then wget "$1" -O "$2"
    else err "Unknown downloader"   # should not reach here
    fi
}

check_apt_install() {
    if [ "$(dpkg-query -l | grep $1 | wc -l)" = 0 ]; then
        run_sudo apt install -y $1
    fi
}

check_dnf_install() {
    if [ "$(dnf list installed | grep $1 | wc -l)" = 0 ]; then
        run_sudo dnf install -y $1
    fi
}

check_yum_install_rpm() {
    if [ "$(dnf list installed | grep $1 | wc -l)" = 0 ]; then
        run_sudo yum install -y $2
    fi
}

check_pacman_install() {
    if [ "$(pacman -Q | grep $1 | wc -l)" = 0 ]; then
        run_sudo pacman -S --noconfirm $1
    fi
}

run_sudo() {
    if ! check_cmd "sudo"
    then $@
    else sudo $@
    fi
}

main "$@" || exit 1
