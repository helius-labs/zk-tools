"""Write pairs of little-endian signed GLV components for matching.rs."""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "2026-11-19-trained-digits541/artifacts"))
from zp_glv513 import P, split

def prepare(source, target):
    data = source.read_bytes()
    if not data or len(data) % 32:
        raise ValueError("expected packed 32-byte scalars")
    classes = {}
    for offset in range(0, len(data), 32):
        x = int.from_bytes(data[offset:offset+32], "little")
        if x >= P:
            raise ValueError("scalar outside BN254 Fr")
        if x:
            classes[min(x, P-x)] = None
    with target.open("wb") as out:
        for x in classes:
            for component in split(x):
                out.write(component.to_bytes(16, "little", signed=True))

if __name__ == "__main__":
    prepare(Path(sys.argv[1]), Path(sys.argv[2]))
