#!/usr/bin/env python3
"""Read-only CLI for offline Pulse Key wearer-verification research.

Inputs: local oura.db + private audit with labels for replay only.
This code never connects to BLE, changes ring firmware, calls a network API,
uses secret pairing keys, or modifies the working Pulse Key daemon.
"""
from __future__ import annotations
import argparse,json,sys
from pathlib import Path
from observations import discover,observe,quality
from verifier import enroll,verify


def main(argv=None):
    parser=argparse.ArgumentParser(description='Pulse Key offline experimental identity verifier (never grants authorization)')
    parser.add_argument('--db',required=True,help='Read-only Oura SQLite event database')
    actions=parser.add_subparsers(dest='command',required=True)
    actions.add_parser('list',help='Inspect raw CVA burst quality, no labels')
    p=actions.add_parser('enroll',help='Freeze owner-only profile')
    p.add_argument('--starts',required=True,nargs='+',type=int,help='>=3 manually selected owner burst starts')
    p.add_argument('--out',required=True)
    p.add_argument('--optical-slack-counts',type=int,default=2)
    p.add_argument('--waveform-slack',type=float,default=1.15)
    p=actions.add_parser('verify',help='Replay one previously unseen captured burst')
    p.add_argument('--start',required=True,type=int)
    p.add_argument('--profile',required=True)
    p=actions.add_parser('backtest',help='Chronological replay with labels from private audit')
    p.add_argument('--audit',required=True)
    p.add_argument('--enroll-starts',nargs='+',type=int,default=[715158,2659172,2661558])
    p.add_argument('--optical-slack-counts',type=int,default=2)
    p.add_argument('--waveform-slack',type=float,default=1.15)
    args=parser.parse_args(argv)
    if args.command=='list':
        output=[]
        for start,end,n in discover(args.db):
            o=observe(args.db,start,end,include_waveform=True)
            output.append({'start':start,'end':end,'packets':n,'difference':o['difference'],
                           'waveform':bool(o['waveform']),'quality_errors':quality(o)})
        print(json.dumps(output,indent=2));return 0
    if args.command=='enroll':
        data=[observe(args.db,s) for s in args.starts]
        profile=enroll(data,optical_slack_counts=args.optical_slack_counts,waveform_slack=args.waveform_slack)
        p=Path(args.out)
        if p.exists():parser.error(f'profile already exists: {p} (refusing overwrite)')
        # The profile contains private derived biometrics; owner-read-only on macOS/Unix.
        import os
        fd=os.open(str(p),os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o600)
        with os.fdopen(fd,'w') as file:json.dump(profile,file,indent=2)
        print(json.dumps({'profile_created':str(p),'enrollment_starts':args.starts,
                          'optical_allowed':[profile['optical']['lower'],profile['optical']['upper']],
                          'waveform_threshold':profile['waveform']['nn_distance_threshold']},indent=2));return 0
    if args.command=='verify':
        profile=json.loads(Path(args.profile).read_text())
        out=verify(profile,observe(args.db,args.start))
        print(json.dumps(out,indent=2));return 0
    if args.command=='backtest':
        audit=json.loads(Path(args.audit).read_text())
        tagged={int(r['ts_start']):r['wearer'] for r in audit['protocol_and_burst_audit']}
        train=[observe(args.db,s) for s in args.enroll_starts]
        if any(tagged.get(s)!='X' for s in args.enroll_starts):parser.error('benchmark enrollment starts must be known X')
        profile=enroll(train,optical_slack_counts=args.optical_slack_counts,waveform_slack=args.waveform_slack)
        cutoff=max(args.enroll_starts)
        records=[]
        for start in sorted(tagged):
            if start in args.enroll_starts or start<=cutoff:continue
            obs=observe(args.db,start)
            result=verify(profile,obs)
            records.append({'wearer':tagged[start],**result})
        counts={}
        for who in ('X','Y'):
            subset=[r for r in records if r['wearer']==who]
            counts[who]={status:sum(x['decision']==status for x in subset) for status in ('ACCEPT','REJECT','INCONCLUSIVE')}
        print(json.dumps({'training_starts':args.enroll_starts,'development_backtest_only':True,
                          'warning':'Threshold slack and features were explored using X/Y history; these are not prospective FAR/FRR estimates.',
                          'counts':counts,'records':records},indent=2));return 0

if __name__=='__main__':sys.exit(main())
