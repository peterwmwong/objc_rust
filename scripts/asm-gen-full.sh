#!/bin/sh

set -e

TARGET_PATH=$1
OUTPUT_PATH="/tmp/asm-gen-full-$2.s"

xcrun llvm-objdump         \
    -s                     \
    --macho                \
    --disassemble          \
    --no-show-raw-insn     \
    --objc-meta-data       \
    --dylibs-used          \
    --reloc "$TARGET_PATH" \
> "$OUTPUT_PATH"
