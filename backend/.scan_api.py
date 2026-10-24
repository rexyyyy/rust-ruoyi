# -*- coding: utf-8 -*-
import os, re, glob

API = r"D:\rust-proj\rust-ruoyi\plus-ui-5.X\src\api"
out = []
for p in sorted(glob.glob(os.path.join(API, "**", "*.ts"), recursive=True)):
    with open(p, encoding="utf-8", errors="replace") as f:
        text = f.read()
    fname = os.path.relpath(p, API)
    for m in re.finditer(r"url:\s*[`'\"]([^`'\"]+)[`'\"]", text):
        out.append((fname, m.group(1)))
for fname, url in out:
    print(f"{fname}\t{url}")
