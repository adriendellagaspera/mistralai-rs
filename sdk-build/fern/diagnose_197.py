#!/usr/bin/env python3
"""Probe whether pinned/latest Fern handles the unmodified #197 OpenAPI forms.

This temporary diagnostic only modifies an ephemeral Fern input; it does not
change published.yaml or generated src.
"""
from __future__ import annotations
import json
import os
import subprocess
import sys
from pathlib import Path
import yaml

ROOT=Path(__file__).resolve().parents[2]
FDIR=ROOT/"sdk-build/fern"
ORIGINAL=yaml.safe_load((ROOT/"sdk-build/openapi/published.yaml").read_text())
COMPAT=yaml.safe_load((FDIR/"compatible.yaml").read_text())
ORIG_SCHEMA=ORIGINAL["components"]["schemas"]
MAPPING={
    "And":("matches","items"),"Or":("matches","items"),"Not":("match",),
}
modes=("baseline","recursive","ipa","responses","all")
log=[]
for mode in modes:
    doc=yaml.safe_load(yaml.safe_dump(COMPAT))
    schemas=doc["components"]["schemas"]
    if mode in ("recursive","all"):
        for name,keys in MAPPING.items():
            node=schemas[name]["properties"]
            original=ORIG_SCHEMA[name]["properties"]
            for key in keys:
                node=node[key]
                original=original[key]
            assert "discriminator" in original and "discriminator" not in node
            node["discriminator"]=original["discriminator"]
    if mode in ("ipa","all"):
        schemas["IPAPhoneme"]=ORIG_SCHEMA["IPAPhoneme"]
    if mode in ("responses","all"):
        # Restore only the wrong mapping from the published original.
        dst=schemas["ResponsesStreamFrame"]
        src=ORIG_SCHEMA["ResponsesStreamFrame"]
        def locate(node):
            if isinstance(node,dict):
                if "oneOf" in node and "discriminator" in node:
                    refs=[x.get("$ref","") for x in node["oneOf"]]
                    if any(x.endswith("/ResponsesErrorEvent") for x in refs):
                        return node
                for x in node.values():
                    hit=locate(x)
                    if hit is not None: return hit
            if isinstance(node,list):
                for x in node:
                    hit=locate(x)
                    if hit is not None:return hit
            return None
        a,b=locate(dst),locate(src)
        assert a is not None and b is not None and a["discriminator"]!=b["discriminator"]
        a["discriminator"]=b["discriminator"]
    (FDIR/"compatible.yaml").write_text(yaml.safe_dump(doc,sort_keys=False))
    command=[
      sys.executable,str(ROOT/"sdk-build/sdk_surface/sdk_surface.py"),"compile",
      "--source",str(FDIR/"compatible.yaml"),"--policy",str(FDIR/"policy.yaml"),
      "--output",str(FDIR/"fern/openapi.json"),"--resolution",str(FDIR/"resolution.json")
    ]
    x=subprocess.run(command,cwd=ROOT,capture_output=True,text=True,timeout=180)
    if x.returncode==0:
        try:
            ir=subprocess.run(["fern","ir","fern-ir.json","--language","rust",
                    "--disable-examples","--from-openapi"],
                cwd=FDIR,capture_output=True,text=True,timeout=180)
            success=ir.returncode==0
            output=ir.stdout+"\n"+ir.stderr
        except subprocess.TimeoutExpired as err:
            success=False;output="TIMEOUT 180 seconds"
    else:
        success=False;output=x.stdout+"\n"+x.stderr
    last=output[-1300:]
    item={"mode":mode,"success":success,"output":last}
    log.append(item)
    print("\n=== "+mode+" PASS="+str(success)+" ===\n"+last,flush=True)
print("\nRESULTS",json.dumps([{"mode":x["mode"],"success":x["success"]} for x in log]),flush=True)
if not log[0]["success"]:
    raise SystemExit("BASELINE FAIL: cannot interpret single-workaround probes")
