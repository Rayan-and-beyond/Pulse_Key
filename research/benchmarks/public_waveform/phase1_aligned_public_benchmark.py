"""Pulse Key offline waveform-only challenge with SNOWBALL public ring PPG.

Privacy: no participant re-identification, raw Oura data, N5-N3 calibration, or unlock actions.
The waveform pipeline is loaded verbatim from the frozen phase-1 feature_extraction.py.
Data selection reuses the pre-existing 163 quality-selected public windows as recorded in
public_features_optical.json. The X reference is read separately on trusted local storage.
"""
from __future__ import annotations
import json, sys, statistics
from collections import defaultdict, Counter
from pathlib import Path
import numpy as np
from scipy.signal import resample_poly

ROOT=Path(__file__).resolve().parent
sys.path.insert(0,str(ROOT/'pulse_key_phase1'))
from feature_extraction import features,distance


def collect_public(raw_dir:Path,selection_path:Path,output_path:Path):
    j=json.loads(selection_path.read_text())
    windows=j['records']; grouped=defaultdict(list)
    for w in windows:grouped[int(w['pid'])].append(w)
    all_rows=[]
    for pid in range(1,21):
        subset=grouped.get(pid,[])
        if not subset:continue
        fn=raw_dir/f'P{pid}_Ring_raw.npz'
        with np.load(fn,allow_pickle=False) as archive:
            times=archive['timestamp']
            starts=[int(np.searchsorted(times,w['time'])) for w in subset]
            del times
            green=archive['ppg_green']
            for w,i in zip(subset,starts):
                seg=green[i:i+3000]
                if len(seg)!=3000 or not np.all(np.isfinite(seg)):
                    continue
                # Exact 100 -> 50 Hz anti-aliased resampling. Leave samples unscaled.
                f=features(resample_poly(seg,1,2))
                all_rows.append(dict(pid=pid,day=int(w['day']),start_ms=int(w['time']),features=f))
            del green
        print(f'public P{pid}: {sum(r["features"] is not None for r in all_rows if r["pid"]==pid)}/{len(subset)} with feature quality',flush=True)
    output_path.write_text(json.dumps({'project':'Pulse Key','extractor':'Phase1 feature_extraction.features EXACT source','source':'SNOWBALL Multisite-PPG raw ring files','selection':'previous public_features_optical.json; 30 s segments; 30 min apart','records':all_rows},separators=(',',':')))
    return all_rows


def evaluate_public(rows, x_templates, x_threshold, x_rise_ms):
    usable=[r for r in rows if r['features'] is not None]
    if not usable:
        raise ValueError('No qualifying public waveform windows; no biometric comparison is possible')
    for r in usable:
        f=r['features'];r['x_distance']=min(distance(f,{'apg_dct_1_16':t}) for t in x_templates)
        r['x_wave_pass']=r['x_distance']<=x_threshold
        r['x_rise_delta']=min(abs(f['t10_50_ms']-q) for q in x_rise_ms)
    accepted=[r for r in usable if r['x_wave_pass']]
    print('\n*** X-vs-public waveform-only challenge ***',flush=True)
    print('selected',len(rows),'quality_passed',len(usable),'participants_usable',len({r['pid'] for r in usable}),'wave_pass',len(accepted),'wave_fail',len(usable)-len(accepted),flush=True)
    print('people_with_any_wave_pass',len({r['pid'] for r in accepted}),flush=True)
    print('score_quantiles',np.percentile([r['x_distance'] for r in usable],[0,5,25,50,75,95,100]).round(4).tolist(),flush=True)
    print('closest public comparisons (anonymized ID; one per person only):',flush=True)
    best=[]
    for pid in sorted({r['pid'] for r in usable}):
        rs=[r for r in usable if r['pid']==pid]
        k=min(rs,key=lambda r:r['x_distance'])
        best.append(dict(pid=pid,windows=len(rs),min_distance=round(k['x_distance'],5),rise_ms=k['features']['t10_50_ms'],wave_pass=k['x_wave_pass']))
    for r in sorted(best,key=lambda r:r['min_distance']):print(r,flush=True)
    # Provide descriptive within-public AUC using each person's earliest usable window as reference.
    cohort=defaultdict(list)
    for r in usable:cohort[r['pid']].append(r)
    refs={pid:min(rs,key=lambda a:a['start_ms']) for pid,rs in cohort.items() if len(rs)>=2}
    for separation in ('same_day','different_day'):
        person_aucs=[];g_total=0;i_total=0
        # Equal weighting across reference identities: each person gets one AUC.
        for pid,ref in refs.items():
            genuine=[distance(ref['features'],x['features']) for x in cohort[pid]
                     if x['start_ms']!=ref['start_ms'] and ((x['day']!=ref['day'])==(separation=='different_day'))]
            impostors=[distance(ref['features'],x['features']) for oid,x in refs.items() if oid!=pid]
            if not genuine or not impostors:continue
            arr=np.asarray(genuine)[:,None]; brr=np.asarray(impostors)[None,:]
            person_aucs.append(float(np.mean(arr<brr)+0.5*np.mean(arr==brr)))
            g_total+=len(genuine);i_total+=len(impostors)
        score=float(np.mean(person_aucs)) if person_aucs else None
        print('within_public',separation,'n_reference_subjects',len(person_aucs),'genuine',g_total,'impostor_reference_pairs',i_total,'subject_mean_AUC',score,flush=True)
    return {'selected':len(rows),'usable':len(usable),'usable_people':len({r['pid'] for r in usable}),
            'passed':len(accepted),'people_any_pass':len({r['pid'] for r in accepted}),
            'threshold':x_threshold,'subject_minimums':best}


if __name__=='__main__':
    import argparse
    pa=argparse.ArgumentParser();pa.add_argument('--raw',required=True);pa.add_argument('--selection',required=True);pa.add_argument('--output',required=True);pa.add_argument('--owner-template',required=True)
    args=pa.parse_args()
    rows=collect_public(Path(args.raw),Path(args.selection),Path(args.output))
    private=json.loads(Path(args.owner_template).read_text())
    summary=evaluate_public(rows,private['templates'],private['threshold'],private['rise_ms_owner'])
    Path(args.output+'.summary.json').write_text(json.dumps(summary,indent=2))
