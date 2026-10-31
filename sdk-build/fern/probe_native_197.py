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
assert mode in ("ipa", "responses")
if mode=="ipa":
    assert a["IPAPhoneme"]!=b["IPAPhoneme"]
    a["IPAPhoneme"]=b["IPAPhoneme"]
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
