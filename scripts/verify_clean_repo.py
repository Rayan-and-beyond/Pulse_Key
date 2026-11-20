#!/usr/bin/env python3
"""Conservative repository preflight; not a replacement for dedicated secret scanning."""
from pathlib import Path
import subprocess, re, sys
root=Path(__file__).resolve().parents[1]
tracked=subprocess.run(['git','ls-files','--cached','--others','--exclude-standard'],cwd=root,text=True,capture_output=True,check=True).stdout.splitlines()
patterns=[
  re.compile(r'(^|/)(?:oura\.db|owner_profile_private\.json|multimodal-audit-private\.json)(?:$|[.-])',re.I),
  re.compile(r'(^|/)(?:\.env(?:\.|$)|id_rsa$|id_ed25519$|credentials?\.json$)',re.I),
  re.compile(r'(?:^|/)(?:private|raw|sessions|data|consent|participants|recordings)/',re.I),
  re.compile(r'\.(?:pem|key|p12|pfx|sqlite|sqlite3|db|pcap|pcapng|h5|npy|npz|parquet)$',re.I),
]
errors=[]
for name in tracked:
  p=root/name
  if any(q.search(name) for q in patterns):errors.append(f'PROHIBITED PATH: {name}')
  if not p.is_file():continue
  if p.stat().st_size>5_000_000:errors.append(f'OVERSIZED FILE: {name}')
  if p.suffix in ('.py','.md','.yml','.yaml','.txt','.json'):
    data=p.read_text(encoding='utf8',errors='replace')
    if re.search(r'-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----',data):errors.append(f'EMBEDDED KEY: {name}')
    if re.search(r'ghp_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{40,}',data):errors.append(f'GH TOKEN PATTERN: {name}')
if errors:
  print('\n'.join(errors),file=sys.stderr);sys.exit(1)
print(f'Privacy preflight passed for {len(tracked)} eligible files (filename/size/basic-token rules).')
