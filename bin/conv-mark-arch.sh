#!/bin/sh

BP=$(dirname "$0")

MARK_ARCH="$1"

case "$MARK_ARCH" in
  aarch64-gnu)
    TARGET_ARCH="aarch64-unknown-linux-gnu"
    ;;
  arm-gnu)
    TARGET_ARCH="arm-unknown-linux-gnueabihf"
    ;;
  armv7-gnu)
    TARGET_ARCH="armv7-unknown-linux-gnueabihf"
    ;;
  i586-gnu)
    TARGET_ARCH="i586-unknown-linux-gnu"
    ;;
  i686-gnu)
    TARGET_ARCH="i686-unknown-linux-gnu"
    ;;
  mips-gnu)
    TARGET_ARCH="mips-unknown-linux-gnu"
    ;;
  mipsel-gnu)
    TARGET_ARCH="mipsel-unknown-linux-gnu"
    ;;
  mips64-gnu)
    TARGET_ARCH="mips64-unknown-linux-gnuabi64"
    ;;
  mips64el-gnu)
    TARGET_ARCH="mips64el-unknown-linux-gnuabi64"
    ;;
  powerpc-gnu)
    TARGET_ARCH="powerpc-unknown-linux-gnu"
    ;;
  powerpc64-gnu)
    TARGET_ARCH="powerpc64-unknown-linux-gnu"
    ;;
  powerpc64le-gnu)
    TARGET_ARCH="powerpc64le-unknown-linux-gnu"
    ;;
  riscv64-gnu)
    TARGET_ARCH="riscv64gc-unknown-linux-gnu"
    ;;
  s390x-gnu)
    TARGET_ARCH="s390x-unknown-linux-gnu"
    ;;
  sparc64-gnu)
    TARGET_ARCH="sparc64-unknown-linux-gnu"
    ;;
  x86_64-gnu)
    TARGET_ARCH="x86_64-unknown-linux-gnu"
    ;;
  aarch64-musl)
    TARGET_ARCH="aarch64-unknown-linux-musl"
    ;;
  arm-musl)
    TARGET_ARCH="arm-unknown-linux-musleabihf"
    ;;
  armv7-musl)
    TARGET_ARCH="armv7-unknown-linux-musleabihf"
    ;;
  i586-musl)
    TARGET_ARCH="i586-unknown-linux-musl"
    ;;
  i686-musl)
    TARGET_ARCH="i686-unknown-linux-musl"
    ;;
  mips-musl)
    TARGET_ARCH="mips-unknown-linux-musl"
    ;;
  mipsel-musl)
    TARGET_ARCH="mipsel-unknown-linux-musl"
    ;;
  mips64-musl)
    TARGET_ARCH="mips64-unknown-linux-muslabi64"
    ;;
  mips64el-musl)
    TARGET_ARCH="mips64el-unknown-linux-muslabi64"
    ;;
  x86_64-musl)
    TARGET_ARCH="x86_64-unknown-linux-musl"
    ;;
  help)
    echo "[usage]$0 arch" 1>&2
    echo "aarch64-gnu arm-gnu armv7-gnu i586-gnu i686-gnu mips-gnu mips64-gnu powerpc-gnu powerpc64-gnu riscv64-gnu s390x-gnu sparc64-gnu x86_64-gnu" 1>&2
    echo "aarch64-musl arm-musl armv7-musl i586-musl i686-musl mips-musl mips64-musl x86_64-musl" 1>&2
    exit
    ;;
  *)
    echo "[usage]$0 arch" 1>&2; exit
    ;;
esac
echo "$TARGET_ARCH"
