"""Read-only adapter for the experimental local Oura SQLite CVA history."""
from __future__ import annotations
import json
import re
import sqlite3
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Optional
from feature_extraction import from_sqlite, features

CAL_RE=re.compile(r'^E:([35])N(\d+)A([\d,]+)$')
REG3_RE=re.compile(r'(?:50|51)([0-9a-f]{2})c2ea1020')
REG5_RE=re.compile(r'(?:61|62)([0-9a-f]{2})c1ea1020')


def conn(db_path):
    if not Path(db_path).is_file():raise FileNotFoundError(db_path)
    return sqlite3.connect('file:'+str(Path(db_path).resolve())+'?mode=ro',uri=True)


def discover(db_path):
    c=conn(db_path)
    try:times=[x[0] for x in c.execute('SELECT ring_timestamp FROM events WHERE tag=129 ORDER BY ring_timestamp,id')]
    finally:c.close()
    groups=[]
    for t in times:
        if not groups or t-groups[-1][1]>20:groups.append([t,t,1])
        else:groups[-1][1]=t;groups[-1][2]+=1
    return groups


def optical_calibration(c,start):
    # Calibration finishes just ahead of the waveform; bound the search to this CVA session.
    before=c.execute('SELECT ring_timestamp,tag,decoded_json FROM events WHERE ring_timestamp BETWEEN ? AND ? AND tag IN (67,97) ORDER BY ring_timestamp,id',(start-140,start)).fetchall()
    candidates=[]
    for ts,tag,raw in before:
        if tag!=67 or not raw:continue
        try:a=json.loads(raw).get('ascii','')
        except ValueError:continue
        m=CAL_RE.fullmatch(a)
        if m:candidates.append((ts,int(m[1]),int(m[2]),int(m[3].replace(',',''))))
    last={}
    for row in candidates:last[row[1]]=row
    if len(last)<2:return {'n3':None,'n5':None,'adc3':None,'adc5':None,'difference':None,'protocol_verified_exact':False}
    t3,_,n3,adc3=last[3];t5,_,n5,adc5=last[5]
    if abs(t5-t3)>40 or start-max(t3,t5)>50 or min(t3,t5)<start-140:
        return {'n3':None,'n5':None,'adc3':None,'adc5':None,'difference':None,'protocol_verified_exact':False}
    # Inspect the actual low-level debug register writes and validate both register paths.
    traces={3:[],5:[]}
    for ts,tag,raw in before:
        if tag!=97 or ts>=max(t3,t5) or not raw:continue
        try:d=json.loads(raw)
        except ValueError:continue
        if d.get('subtype')!=0x33:continue
        h=d.get('raw','').lower()
        for pattern,ch in ((REG3_RE,3),(REG5_RE,5)):
            m=pattern.search(h)
            if m:traces[ch].append((ts,int(m[1],16)))
    # Verify complete register-step sets, allowing asynchronous packet ordering.
    statuses=[]
    for ch,n,stamp in ((3,n3,t3),(5,n5,t5)):
        seq=[v for t,v in traces[ch] if stamp-140<=t<=stamp]
        expected=list(range(1,2+4*n,4))
        statuses.append(len(seq)==len(expected) and sorted(seq)==expected)
    return {'n3':n3,'n5':n5,'adc3':adc3,'adc5':adc5,
            'difference':n5-n3,'protocol_verified_exact':all(statuses)}


def observe(db_path,start,stop=None,include_waveform=True):
    groups=discover(db_path)
    matching=[g for g in groups if g[0]==int(start)]
    if not matching:raise ValueError(f'CVA burst start {start} absent')
    s,e,n=matching[0]
    if stop is not None and e!=int(stop):raise ValueError(f'end mismatch for {s}: got {e}, expected {stop}')
    c=conn(db_path)
    try:
        optical=optical_calibration(c,s)
        scan=c.execute('SELECT COUNT(*) FROM events WHERE tag=130 AND ring_timestamp BETWEEN ? AND ?',(s,e)).fetchone()[0]
        moves=[]
        for (raw,) in c.execute('SELECT decoded_json FROM events WHERE tag=71 AND ring_timestamp BETWEEN ? AND ?',(s,e)):
            try:moves.append(int(json.loads(raw).get('motion_seconds',0)))
            except (TypeError,ValueError,AttributeError):pass
    finally:c.close()
    info={'start':s,'end':e,'cva_packets':n,'mid_burst_scan_count':scan,
          'observed_motion_s':max(moves,default=0)}
    info.update(optical)
    if include_waveform:
        try:
            samples=from_sqlite(db_path,s,e)
            info['samples_n']=len(samples)
            info['waveform']=features(samples)
        except ValueError as exc:
            info['samples_n']=0;info['waveform']=None;info['waveform_error']=str(exc)
    return info


def quality(info,require_protocol=True,require_waveform=True):
    reasons=[]
    if info.get('difference') is None:reasons.append('missing_optical_calibration')
    if require_protocol and not info.get('protocol_verified_exact'):reasons.append('uncorroborated_calibration_protocol')
    if info.get('mid_burst_scan_count',0):reasons.append('mid_burst_scan')
    if info.get('observed_motion_s',0)>0:reasons.append('motion_during_capture')
    if require_waveform:
        if not info.get('waveform'):reasons.append('poor_or_missing_waveform')
        if info.get('samples_n',0)<1300:reasons.append('short_capture')
    return reasons
