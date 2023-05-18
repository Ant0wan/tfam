#!/bin/sh
set -o errexit
get_machine_arch () {
	machine_arch=""
	case $(uname -m) in
		x86_64)   machine_arch="x86_64" ;;
		arm64)    machine_arch="aarch64" ;;
		aarch64)  machine_arch="aarch64" ;;
	esac
	echo $machine_arch
}
arch=$(get_machine_arch)
case "$(uname -s)" in
	Darwin*)
		bin="${arch}-apple-darwin"
		;;
	MINGW64*)
		bin="${arch}-pc-windows-gnu"
		;;
	MSYS_NT*)
		bin="${arch}-pc-windows-gnu"
		;;
	*)
		bin="${arch}-unknown-linux-musl"
		;;
esac
echo "bin=tfam_$bin"
download_path=$(mktemp -d -t tfam.XXXXXXXXXX)
download_executable="${download_path}/tfam"
echo "Downloading tfam latest version"
wget --quiet "https://github.com/Ant0wan/tfam/releases/latest/download/tfam_${bin}" -O "${download_executable}"
echo "Downloaded successfully"
dest="${TFEXE_INSTALL_PATH:-/usr/local/bin}/"
echo "Installing tfam to ${dest} ..."
if [ -w "$dest" ]; then SUDO=""; else
	SUDO="sudo";
fi
$SUDO mkdir -p "$dest"
$SUDO install -c -v "${download_executable}" "$dest"
retVal=$?
if [ $retVal -ne 0 ]; then
	echo "Failed to install tfam"
	exit $retVal
fi
echo "Cleaning temporary downloaded files directory ${download_path} ..."
rm -rf "${download_path}"
