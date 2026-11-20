# Corrected v2 source status

This is the corrected 3 November 2026 offline research code. Its internal `README.md` originated in the October 30 v1 package and carries historical wording; read `RESEARCH_CORRECTION_2026-11-03.md` and the repository's current status first. The v2 decoder patch and optical parser were independently checked against historical private data in the research reports. The standalone tests pass 14 with 6 additional DB-backed tests skipped when no private data is available. It cannot unlock anything and returns `authorized:false`.
