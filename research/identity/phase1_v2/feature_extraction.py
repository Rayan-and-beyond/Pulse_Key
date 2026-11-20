"""Offline experimental CVA PPG feature extraction. No BLE or device writes."""
from __future__ import annotations
import json
import sqlite3
from pathlib import Path
import numpy as np
from scipy.signal import butter, sosfiltfilt, find_peaks, savgol_filter, resample_poly
from scipy.fft import dct

FS = 50.0


def reconstruct(events):
    """Reconstruct Oura CVA PPG from decoded packet-relative sample values.

    The Rust event decoder has already accumulated the within-packet deltas:
    non-marker ``ppg_samples`` are offsets from the value at the *start* of
    that packet, not independent increments.  Add each to the same packet
    anchor; advance the anchor to the packet's last absolute value.

    This corrects the v1 Library double-integration bug.  Historical v1
    templates and thresholds MUST be recomputed before offline replay.
    """
    samples=[]; anchor=None; abs_frames=0
    for frame in events:
        d=json.loads(frame[0]) if isinstance(frame[0],str) else frame[0]
        vals=d.get('ppg_samples')
        if not isinstance(vals,list) or not vals:continue
        if d.get('absolute_markers',0):
            if abs_frames: raise ValueError('multiple absolute-marker packets')
            samples.extend(float(v) for v in vals)
            anchor=int(vals[-1]);abs_frames+=1
        elif anchor is not None:
            packet_anchor=anchor
            packet_values=[float(packet_anchor+int(v)) for v in vals]
            samples.extend(packet_values)
            anchor=int(packet_values[-1])
        else:
            raise ValueError('relative packet before absolute marker')
    if abs_frames!=1:raise ValueError(f'expected 1 absolute frame, got {abs_frames}')
    return np.asarray(samples,dtype=np.float64)


def from_sqlite(db_path, start, stop):
    con=sqlite3.connect(f'file:{Path(db_path).resolve()}?mode=ro',uri=True)
    try:
        rows=con.execute('SELECT decoded_json FROM events WHERE tag=129 AND ring_timestamp BETWEEN ? AND ? AND decoded_json IS NOT NULL ORDER BY ring_timestamp,id',(int(start),int(stop))).fetchall()
    finally:con.close()
    return reconstruct(rows)


def rise_time_ms(x,lo=.1,hi=.5,sample_hz=250.0):
    """10%-50% crossing along one upward-going systolic pulse segment."""
    y=np.maximum.accumulate(np.asarray(x,dtype=float))
    delta=y[-1]-y[0]
    if delta<=0:return None
    def crossing(level):
        target=y[0]+delta*level
        j=int(np.searchsorted(y,target))
        if j==0 or j>=len(y):return None
        return (j-1)+(target-y[j-1])/(y[j]-y[j-1]+1e-12)
    a,b=crossing(lo),crossing(hi)
    return (b-a)*1000/sample_hz if a is not None and b is not None else None


def features(samples):
    """Return morphology and rise features, or None if quality gate fails."""
    if len(samples)<850 or np.any(~np.isfinite(samples)):return None
    # Upsample for smooth crossing *estimation*, not new physical information.
    x=resample_poly(np.asarray(samples,dtype=float)-np.median(samples),5,1)
    fs=FS*5
    sos=butter(3,[.5,12],fs=fs,btype='bandpass',output='sos')
    x=sosfiltfilt(sos,x)
    xstd=float(np.std(x))
    if xstd<1e-3:return None
    best=None
    for polarity in (1,-1):
        y=polarity*x
        pk,_=find_peaks(y,distance=int(.46*fs),prominence=.42*xstd)
        if len(pk)<13:continue
        valleys=[]; rises=[]; crests=[]
        for p in pk:
            lo=max(0,p-int(.34*fs));hi=p-int(.08*fs)
            if hi<=lo:continue
            v=lo+int(np.argmin(y[lo:hi]))
            length=p-v
            if not int(.10*fs)<=length<=int(.34*fs):continue
            amplitude=y[p]-y[v]
            if amplitude<.32*xstd:continue
            rt=rise_time_ms(y[v:p+1],sample_hz=fs)
            if rt is None or not 5<=rt<=140:continue
            rises.append(float(rt));crests.append(length*1000/fs);valleys.append(v)
        beats=[]
        for a,b in zip(valleys[:-1],valleys[1:]):
            if not int(.42*fs)<=b-a<=int(1.5*fs):continue
            beat=y[a:b]
            n=np.ptp(beat)
            if n<.3*xstd:continue
            aligned=np.interp(np.linspace(0,len(beat)-1,128),np.arange(len(beat)),beat)
            aligned=(aligned-aligned.min())/(np.ptp(aligned)+1e-9)
            beats.append(aligned)
        if len(beats)<12 or len(rises)<15:continue
        median=np.median(beats,axis=0)
        corr=[]
        for b in beats:
            cc=np.corrcoef(b,median)[0,1]
            if np.isfinite(cc):corr.append(cc)
        if not corr:continue
        coherence=float(np.median(corr))
        if coherence<.50:continue
        apg=savgol_filter(median,13,3,deriv=2)
        coeff=dct(apg,type=2,norm='ortho')[1:17]
        scale=np.linalg.norm(coeff)
        if scale<1e-9:continue
        coeff/=scale
        f={'apg_dct_1_16':[round(float(v),9) for v in coeff],
           't10_50_ms':round(float(np.median(rises)),3),
           'crest_ms':round(float(np.median(crests)),3),
           'n_beats':len(beats),'n_rise':len(rises),'coherence':round(coherence,4),'polarity':int(polarity)}
        if best is None or (coherence*min(len(beats),40))>(best['coherence']*min(best['n_beats'],40)):best=f
    return best


def distance(a,b):
    av=np.array(a['apg_dct_1_16']);bv=np.array(b['apg_dct_1_16'])
    return float(np.linalg.norm(av-bv))
