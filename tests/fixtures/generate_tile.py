#!/usr/bin/env python3
"""Write tests/fixtures/tile.png (32×16) for iso_map.tmx."""

from __future__ import annotations

import struct
import zlib
from pathlib import Path

OUT = Path(__file__).with_name("tile.png")


def write_png(path: Path, width: int, height: int) -> None:
    def chunk(tag: bytes, data: bytes) -> bytes:
        crc = zlib.crc32(tag + data) & 0xFFFFFFFF
        return struct.pack(">I", len(data)) + tag + data + struct.pack(">I", crc)

    row = bytes([0, 0, 128, 64, 255] * width)
    raw = b"".join(row for _ in range(height))
    ihdr = struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0)
    png = (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", ihdr)
        + chunk(b"IDAT", zlib.compress(raw))
        + chunk(b"IEND", b"")
    )
    path.write_bytes(png)


if __name__ == "__main__":
    write_png(OUT, 32, 16)
