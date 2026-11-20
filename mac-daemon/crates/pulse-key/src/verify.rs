use crate::config::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Beat {
    pub bpm: f64,
    pub ibi_ms: f64,
}

impl Beat {
    pub fn is_plausible(&self) -> bool {
        self.ibi_ms >= IBI_MIN_MS
            && self.ibi_ms <= IBI_MAX_MS
            && self.bpm >= BPM_MIN
            && self.bpm <= BPM_MAX
    }
}

pub struct CeremonyCtx<'a> {
    pub authed: bool,
    pub tap_after_arm: bool,
    pub beats: &'a [Beat],
}

#[derive(Debug, Clone, PartialEq)]
pub enum Verdict {
    Pass,
    Fail(String),
    Skipped,
}

pub trait Verifier: Send + Sync {
    fn name(&self) -> &'static str;
    fn verify(&self, ctx: &CeremonyCtx) -> Verdict;
}

pub struct GestureVerifier;

impl Verifier for GestureVerifier {
    fn name(&self) -> &'static str {
        "gesture"
    }
    fn verify(&self, ctx: &CeremonyCtx) -> Verdict {
        if ctx.tap_after_arm {
            Verdict::Pass
        } else {
            Verdict::Fail("tap not confirmed".into())
        }
    }
}

pub struct PossessionVerifier;

impl Verifier for PossessionVerifier {
    fn name(&self) -> &'static str {
        "possession"
    }
    fn verify(&self, ctx: &CeremonyCtx) -> Verdict {
        if ctx.authed {
            Verdict::Pass
        } else {
            Verdict::Fail("ring not authenticated".into())
        }
    }
}

pub struct LivenessVerifier;

impl Verifier for LivenessVerifier {
    fn name(&self) -> &'static str {
        "liveness"
    }
    fn verify(&self, ctx: &CeremonyCtx) -> Verdict {
        if ctx.beats.is_empty() {
            return Verdict::Fail("no pulse detected".into());
        }
        let valid = ctx
            .beats
            .iter()
            .filter(|b| b.is_plausible())
            .count();
        if valid >= MIN_BEATS {
            Verdict::Pass
        } else {
            Verdict::Fail("pulse implausible".into())
        }
    }
}

pub struct IdentityStubVerifier;

impl Verifier for IdentityStubVerifier {
    fn name(&self) -> &'static str {
        "identity"
    }
    fn verify(&self, _ctx: &CeremonyCtx) -> Verdict {
        Verdict::Skipped
    }
}

pub struct VerifierReport {
    pub passed: bool,
    pub fail_reason: Option<String>,
    pub details: Vec<(&'static str, Verdict)>,
}

pub struct VerifierRegistry {
    verifiers: Vec<Box<dyn Verifier>>,
}

impl VerifierRegistry {
    pub fn new() -> Self {
        Self {
            verifiers: vec![
                Box::new(GestureVerifier),
                Box::new(PossessionVerifier),
                Box::new(LivenessVerifier),
                Box::new(IdentityStubVerifier),
            ],
        }
    }

    pub fn run(&self, ctx: &CeremonyCtx) -> VerifierReport {
        let mut fail_reason = None;
        let mut details = Vec::new();
        for v in &self.verifiers {
            let verdict = v.verify(ctx);
            if let Verdict::Fail(reason) = &verdict {
                if fail_reason.is_none() {
                    fail_reason = Some(reason.clone());
                }
            }
            details.push((v.name(), verdict));
        }
        VerifierReport { passed: fail_reason.is_none(), fail_reason, details }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn beats(n: usize) -> Vec<Beat> {
        (0..n).map(|_| Beat { bpm: 72.0, ibi_ms: 833.0 }).collect()
    }

    #[test]
    fn a_clean_ceremony_passes() {
        let b = beats(5);
        let ctx = CeremonyCtx { authed: true, tap_after_arm: true, beats: &b };
        let r = VerifierRegistry::new().run(&ctx);
        assert!(r.passed, "{:?}", r.details);
        assert!(r.fail_reason.is_none());
    }

    #[test]
    fn no_tap_fails_with_reason() {
        let b = beats(5);
        let ctx = CeremonyCtx { authed: true, tap_after_arm: false, beats: &b };
        let r = VerifierRegistry::new().run(&ctx);
        assert_eq!(r.fail_reason.as_deref(), Some("tap not confirmed"));
    }

    #[test]
    fn no_pulse_fails_with_no_pulse_reason() {
        let ctx = CeremonyCtx { authed: true, tap_after_arm: true, beats: &[] };
        assert_eq!(
            VerifierRegistry::new().run(&ctx).fail_reason.as_deref(),
            Some("no pulse detected")
        );
    }

    #[test]
    fn too_few_beats_fail_implausible() {
        let b = beats(2);
        let ctx = CeremonyCtx { authed: true, tap_after_arm: true, beats: &b };
        assert_eq!(
            VerifierRegistry::new().run(&ctx).fail_reason.as_deref(),
            Some("pulse implausible")
        );
    }

    #[test]
    fn out_of_range_beats_do_not_count() {
        let bad = vec![
            Beat { bpm: 72.0, ibi_ms: 833.0 },
            Beat { bpm: 30.0, ibi_ms: 2000.0 },
            Beat { bpm: 210.0, ibi_ms: 285.0 },
            Beat { bpm: 30.0, ibi_ms: 2000.0 },
        ];
        let ctx = CeremonyCtx { authed: true, tap_after_arm: true, beats: &bad };
        assert_eq!(
            VerifierRegistry::new().run(&ctx).fail_reason.as_deref(),
            Some("pulse implausible")
        );
    }

    #[test]
    fn unauthenticated_ring_fails_possession() {
        let b = beats(5);
        let ctx = CeremonyCtx { authed: false, tap_after_arm: true, beats: &b };
        assert_eq!(
            VerifierRegistry::new().run(&ctx).fail_reason.as_deref(),
            Some("ring not authenticated")
        );
    }

    #[test]
    fn identity_stub_is_skipped() {
        let b = beats(5);
        let ctx = CeremonyCtx { authed: true, tap_after_arm: true, beats: &b };
        let r = VerifierRegistry::new().run(&ctx);
        let (_, verdict) = r.details.iter().find(|(n, _)| *n == "identity").unwrap();
        assert_eq!(*verdict, Verdict::Skipped);
    }
}
