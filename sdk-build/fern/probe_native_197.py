#!/usr/bin/env python3
"""Prepare a temporary Fern input with one #197 shim removed."""
import os
from pathlib import Path
import yaml

ROOT=Path(__file__).resolve().parents[2]
FDIR=ROOT/"sdk-build/fern"
schema=yaml.safe_load((FDIR/"compatible.yaml").read_text())
raw=yaml.safe_load((ROOT/"sdk-build/openapi/published.yaml").read_text())
a=schema["components"]["schemas"]
b=raw["components"]["schemas"]
mode=os.environ["FERN_197_PROBE_MODE"]
assert mode in ("ipa", "ipa-named", "responses")
if mode in ("ipa", "ipa-named"):
    assert a["IPAPhoneme"]!=b["IPAPhoneme"]
    a["IPAPhoneme"]=b["IPAPhoneme"]
    if mode=="ipa-named":
        values=a["IPAPhoneme"]["enum"]
        assert len(set(values))==len(values)
        import re
        a["IPAPhoneme"]["x-fern-enum"]={
            value: {"name": "Ipa" + "".join(f"U{ord(ch):04X}" for ch in value)}
            for value in values if not re.fullmatch(r"[a-zA-Z]+",value)
        }
        print("Named IPA variants:",len(a["IPAPhoneme"]["x-fern-enum"]),flush=True)
if mode=="responses":
    def find(node):
        if isinstance(node,dict):
            if "oneOf" in node and "discriminator" in node:
                if any(x.get("$ref","").endswith("/ResponsesErrorEvent")
                       for x in node["oneOf"] if isinstance(x,dict)): return node
            for x in node.values():
                r=find(x)
                if r is not None: return r
        elif isinstance(node,list):
            for x in node:
                r=find(x)
                if r is not None:return r
        return None
    dst=find(a["ResponsesStreamFrame"])
    src=find(b["ResponsesStreamFrame"])
    assert dst is not None and src is not None
    assert dst["discriminator"]!=src["discriminator"]
    dst["discriminator"]=src["discriminator"]
(FDIR/"compatible.yaml").write_text(yaml.safe_dump(schema,sort_keys=False))
print("Original #197 variant restored:",mode,flush=True)
