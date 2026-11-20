"""Experimental wearer-verification scoring. OFFLINE REPLAY ONLY.

Never use decisions from this module to unlock a vault or grant account access.
No external I/O. Frozen owner-only enrollment template, quality-gated decisions.
"""
from __future__ import annotations
from datetime import datetime,timezone
from feature_extraction import distance
from observations import quality

PROFILE_SCHEMA='pulse-key-offline-identity-v2-corrected-ppg'


def enroll(observations, *, optical_slack_counts=2, waveform_slack=1.15):
    """Fit from owner observations only; caller must select chronological training captures."""
    if len(observations)<3:raise ValueError('minimum three independently captured owner bursts')
    if len({o['start'] for o in observations})!=len(observations):raise ValueError('duplicate owner recordings')
    for o in observations:
        errors=quality(o,require_protocol=True,require_waveform=True)
        if errors:raise ValueError(f'cannot enroll {o["start"]}: {errors}')
    if not isinstance(optical_slack_counts,int) or optical_slack_counts<0 or optical_slack_counts>6:raise ValueError('invalid optical slack')
    if not (1<=waveform_slack<=3):raise ValueError('invalid waveform threshold multiplier')
    diffs=[o['difference'] for o in observations]
    raw_f=[o['waveform'] for o in observations]
    loo=[min(distance(f,g) for j,g in enumerate(raw_f) if i!=j) for i,f in enumerate(raw_f)]
    wav_threshold=max(loo)*waveform_slack
    return {
        'schema':PROFILE_SCHEMA,
        'created_utc':datetime.now(timezone.utc).isoformat(),
        'research_use_only':True,
        'owner_reference_starts':[o['start'] for o in observations],
        'optical':{
            'feature':'N5_minus_N3',
            'owner_observations':diffs,
            'lower':min(diffs)-optical_slack_counts,
            'upper':max(diffs)+optical_slack_counts,
            'slack_counts':optical_slack_counts,
            'note':'Slack chosen during exploratory research; cannot claim an untouched prospective test.'
        },
        'waveform':{
            'feature':'custom_APG_DCT16_v2_corrected_reconstruction',
            'templates':[f['apg_dct_1_16'] for f in raw_f],
            'rise_ms_owner':[f['t10_50_ms'] for f in raw_f],
            'loo_nn_distances':[round(x,5) for x in loo],
            'nn_distance_threshold':round(wav_threshold,6),
            'slack_multiplier':waveform_slack,
            'note':'Independent extractor; not a byte-for-byte reproduction of multimodal_audit.py.'
        },
        'guardrails':{'no_unlock':True,'quality_fail_inconclusive':True,'protocol_binary_corrob_required':True,'mid_scan_disallowed':True,'motion_disallowed':True}
    }


def verify(profile,observation):
    if profile.get('schema')!=PROFILE_SCHEMA or not profile.get('research_use_only'):
        raise ValueError('unsupported or unsafe profile')
    if observation['start'] in profile['owner_reference_starts']:
        # Enrollment recordings are never counted as independently evaluated probes.
        return {'start':observation['start'],'decision':'INCONCLUSIVE','reason':'enrollment_recording','authorized':False}
    problems=quality(observation,require_protocol=True,require_waveform=True)
    if problems:
        # Preserve observability for research, without promoting poor captures to identity verdicts.
        n=observation.get('difference')
        o=profile['optical']
        optical_diag={'difference':n,'within_owner_envelope':o['lower']<=n<=o['upper']} if n is not None else None
        return {'start':observation['start'],'decision':'INCONCLUSIVE','reason':'quality_gate',
                'quality_errors':problems,'optical_diagnostic_only':optical_diag,'authorized':False}
    n=observation['difference'];o=profile['optical']
    optical_pass=o['lower']<=n<=o['upper']
    x={'apg_dct_1_16':observation['waveform']['apg_dct_1_16']}
    minimum=min(distance(x,{'apg_dct_1_16':v}) for v in profile['waveform']['templates'])
    waveform_pass=minimum<=profile['waveform']['nn_distance_threshold']
    decision='ACCEPT' if optical_pass and waveform_pass else 'REJECT'
    return {'start':observation['start'],'decision':decision,'authorized':False,
            'optical':{'pass':optical_pass,'difference':n,'allowed':[o['lower'],o['upper']]},
            'waveform':{'pass':waveform_pass,'distance':round(minimum,6),'threshold':profile['waveform']['nn_distance_threshold'],
                        'rise_ms':observation['waveform']['t10_50_ms']},
            'note':'Research decision for offline replay; never authorization or verified FAR/FRR.'}
