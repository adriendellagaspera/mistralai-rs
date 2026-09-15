from pathlib import Path

path = Path("codegen/patches/generator.patch")
lines = path.read_text().splitlines()
path.write_text("\n".join(line.rstrip() for line in lines) + "\n")
