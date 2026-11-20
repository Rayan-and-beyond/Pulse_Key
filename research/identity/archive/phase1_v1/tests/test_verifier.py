import json
import os
import sys
from pathlib import Path
import pytest
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT))
from verifier import enroll,verify
from observations import observe,discover,quality,optical_calibration,conn
from feature_extraction import reconstruct,distance,features

DB=Path(os.environ.get('PULSE_KEY_TEST_DB','/does/not/exist/oura.db')).expanduser()
AUDIT=Path(os.environ.get('PULSE_KEY_TEST_AUDIT','/does/not/exist/multimodal-audit-private.json')).expanduser()


def demo_f(v=0,base=1):
    arr=[0.0]*16
    arr[0]=float(base)
    return {'apg_dct_1_16':arr,'t10_50_ms':35.0+v,'n_beats':27,'coherence':.9}


def demo_obs(start,diff, wave=None,**extra):
    o={'start':start,'end':start+299,'difference':diff,'protocol_verified_exact':True,
       'mid_burst_scan_count':0,'observed_motion_s':0,'samples_n':1500,'waveform':wave or demo_f()}
    o.update(extra)
    return o


def standard_profile():
    return enroll([demo_obs(101,-2,demo_f(base=1)),demo_obs(201,-3,demo_f(base=.95)),
                   demo_obs(301,-2,demo_f(base=.90))])


def test_reconstruct_mixed_marker():
    frames=[({'ppg_samples':[100,110],'absolute_markers':1},),
            ({'ppg_samples':[3,-4,2],'absolute_markers':0},)]
    assert reconstruct(frames).tolist()==[100.,110.,113.,109.,111.]


def test_reconstruct_requires_absolute_frame():
    with pytest.raises(ValueError):reconstruct([({'ppg_samples':[1,2],'absolute_markers':0},)])


def test_enrollment_requires_three():
    with pytest.raises(ValueError):enroll([demo_obs(101,-2),demo_obs(201,-3)])


def test_enrollment_rejects_duplicate():
    with pytest.raises(ValueError):enroll([demo_obs(101,-2),demo_obs(201,-3),demo_obs(101,-2)])


def test_enrollment_rejects_bad_quality():
    with pytest.raises(ValueError):enroll([demo_obs(101,-2),demo_obs(201,-3),demo_obs(301,-2,mid_burst_scan_count=1)])


def test_enrollment_rejects_invalid_slack():
    with pytest.raises(ValueError):enroll([demo_obs(101,-2),demo_obs(201,-3),demo_obs(301,-2)],optical_slack_counts=-1)


def test_profile_exposes_no_authorization():
    p=standard_profile()
    assert p['research_use_only'] is True
    assert p['guardrails']['no_unlock'] is True


def test_accept_clean_probe():
    assert verify(standard_profile(),demo_obs(401,0))['decision']=='ACCEPT'


def test_reject_impersonation_even_when_waveform_passes():
    r=verify(standard_profile(),demo_obs(401,1))
    assert r['decision']=='REJECT'
    assert r['waveform']['pass'] is True
    assert r['optical']['pass'] is False
    assert r['authorized'] is False


def test_scan_or_motion_is_inconclusive():
    for change in ({'mid_burst_scan_count':1},{'observed_motion_s':1},{'protocol_verified_exact':False},
                   {'samples_n':200},{'waveform':None}):
        assert verify(standard_profile(),demo_obs(401,0,**change))['decision']=='INCONCLUSIVE'


def test_owner_enrollment_recording_never_replayed_as_test():
    assert verify(standard_profile(),demo_obs(101,-2))['decision']=='INCONCLUSIVE'


def test_profile_must_be_offline_schema():
    p=standard_profile();p['research_use_only']=False
    with pytest.raises(ValueError):verify(p,demo_obs(401,0))


@pytest.mark.skipif(not DB.exists(),reason='private DB missing')
def test_raw_database_discovers_15_sessions():
    assert len(discover(DB))==15


@pytest.mark.skipif(not (DB.exists() and AUDIT.exists()),reason='private raw evidence not provided')
def test_independent_calibration_decode_matches_agent_all_15():
    audit=json.loads(AUDIT.read_text())
    for row in audit['protocol_and_burst_audit']:
        o=observe(DB,row['ts_start'],include_waveform=False)
        assert o['difference']==row['n5_minus_n3']
        assert o['n3']==row['n3'] and o['n5']==row['n5']
        assert o['mid_burst_scan_count']==row['mid_burst_scan_0x82']
        assert o['observed_motion_s']==row['observed_motion_s']


@pytest.mark.skipif(not DB.exists(),reason='private DB missing')
def test_chronological_replay_actual_data():
    starts=[715158,2659172,2661558]
    p=enroll([observe(DB,s) for s in starts])
    assert verify(p,observe(DB,2699937))['decision']=='ACCEPT'
    for ts in (2703303,2708139):
        result=verify(p,observe(DB,ts))
        assert result['decision']=='REJECT'
        assert result['waveform']['pass'] is True
        assert result['optical']['pass'] is False
        assert result['authorized'] is False


@pytest.mark.skipif(not DB.exists(),reason='private DB missing')
def test_no_optical_slack_later_owner_rejected():
    p=enroll([observe(DB,s) for s in (715158,2659172,2661558)],optical_slack_counts=0)
    result=verify(p,observe(DB,2699937))
    assert result['decision']=='REJECT'
    assert result['optical']['pass'] is False


@pytest.mark.skipif(not DB.exists(),reason='private DB missing')
def test_embedded_register_packet_is_detected():
    # One register write appears inside a longer subtype-0x33 packet.
    # The prior audit marked this non-exact, although the full sequence exists.
    o=observe(DB,2696276,include_waveform=False)
    assert o['protocol_verified_exact'] is True
    assert o['mid_burst_scan_count']>0
    assert o['observed_motion_s']>0
