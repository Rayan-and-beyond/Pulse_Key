# Security policy and responsible research

## Secrets and human biometric data

Never commit or push ring pairing/authentication keys, credentials, OAuth tokens, original `oura.db`, locally captured private raw or derived pulse sequences, private enrollment profiles, consent records, participant-identifying metadata, serial numbers or host-device addresses. Private repositories still expose data to GitHub's hosting and access control; sensitive human-subject samples should remain encrypted and local.

The Git tree contains source code, synthetic/unit fixtures, aggregate scientific reports and the provenance ZIPs that were checked for excluded private artifacts. Separately collected, already-public, de-identified third-party recordings may be made available as GitHub Release attachments under their original attribution and non-commercial license. This authorization does not cover locally collected or private participant recordings.

## Threat model

**Attack surfaces:** a stolen ring worn by another person, captured/replayed BLE messages, relay of on-body signals, forged sensor events, shoulder-surfed intent patterns, compromised local host, malformed decoder packets and accidental authorization during missing data. Assess claimed freshness, origin authentication, trust boundaries and human user intent separately.

**Risk posture:** the research classifier cannot authorize live transactions and returns `authorized:false`. Its `INCONCLUSIVE` outcome must never be counted as a verified impostor rejection. Current retrospective samples do not provide security-grade false acceptance or false rejection statistics.

## Safe workflow

- Use owned or expressly consented devices and participants.
- Keep the paired old research Ring 4 separate from any new/sealed ring.
- Avoid factory resets, firmware flashing or destructive writes during prospective data collection.
- Freeze configs and hash raw/local model artifacts; commit only aggregate reproducible outcomes.
- Review dependencies and run `python3 scripts/verify_clean_repo.py` before staging or pushing.
- Leave identity gating disabled in the live daemon until independent prospective results warrant it.

## Disclosure

For any verified device vulnerability, follow coordinated disclosure to the manufacturer before releasing exploit-level detail or customer-impacting proof.
