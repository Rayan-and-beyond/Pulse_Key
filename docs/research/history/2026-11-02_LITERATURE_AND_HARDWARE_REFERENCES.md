# External Scientific Leads and Hardware References (as of 2026-11-02)

This is a curated reading and transferability guide, not a claim the project reproduced the published experiments. Prefer primary publications and actual Oura decode evidence.

## Oura hardware and sensor access

- **TechInsights, Oura Ring Gen 4 Teardown** (2025): https://www.techinsights.com/blog/oura-ring-gen-4-teardown . Reports two multicolor LEDs, three photodiodes, up to 18 optical signal pathways, and an **Analog Devices MAX86178F** AFE. This is a published hardware teardown, not proof that all pathways can be read through custom BLE.
- **Oura Smart Sensing** vendor descriptions and claimed **SPIE 2026 paper**, `Smart sensing in Oura Ring 4` (agent cites Proc. SPIE 13845). Verify full bibliographic details; do not assert exact sensor-path mapping from title alone.
- **Community reverse engineering, open_ring protocol:** https://github.com/LogosIsLife/open_ring/blob/master/PROTOCOL.md . Useful for packet hypotheses; validate against project captures and account for firmware 2.12.5 changes.
- **AFE datasheet ambiguity:** Mac agent cites MAX86171. TechInsights identifies MAX86178F on Ring 4. Obtain the exact applicable datasheet before treating register bits or ADC ceiling as verified physical facts.

## Biometric papers

- **Wei Shao et al., 'Know Me by My Pulse: Toward Practical Continuous Authentication on Wearable Devices via Wrist-Worn PPG,' NDSS 2026.** https://www.ndss-symposium.org/ndss-paper/know-me-by-my-pulse-toward-practical-continuous-authentication-on-wearable-devices-via-wrist-worn-ppg/ . DOI: `10.14722/ndss.2026.231087`. Reports a 25 Hz **four-channel** wrist PPG system, 26-person own dataset, EER **2.76%**, FAR **0.48%**, FRR **11.77%** at their evaluation protocol. Neither sensor geometry, channels nor population match our Oura 50 Hz reconstructed single CVA waveform; the numbers **do not** apply to Pulse Key.
- **TrueHeart (Zhao et al., INFOCOM 2020)** https://web.eecs.utk.edu/~jliu/publications/zhao2020trueheart.pdf . Useful for fiducial extraction, rate and APG analyses; results are for another device and training protocol.
- **Apple Fusion-ID (ICASSP 2022)** https://machinelearning.apple.com/research/photoplethysmography-motion-sensor-fusion . Motion + PPG research under different sensors. Actual simultaneous availability matters.
- **Alecci et al. 2025, 'What the Heart Can(not) Tell'** https://pmc.ncbi.nlm.nih.gov/articles/PMC12736706/ . Caution about leakage and changing sessions in PPG identity evaluation. Verify exact experiments before transferring numerical generalizations.
- **Sancho et al. 2018, PPG authentication** https://pmc.ncbi.nlm.nih.gov/articles/PMC5981424/ . Cross-session generalization matters.
- **SNOWBALL Multisite-PPG dataset** https://huggingface.co/datasets/snowballlab/Multisite-PPG . Different ring hardware; lacks proprietary Oura optical calibration; review CC BY-NC and reidentification restrictions before using public physiological streams.

## Do not substitute literature for experiment

- Sample rate alone does not establish identity performance. Multiple independent channels may add substantial information unavailable from the Ring 4 BLE stream.
- Ring hardware marketing and teardown describe *physical capability*, while custom BLE logs constrain *data access*.
- Within-session impressive published biometric error rates can fail under new days, new users, contact changes and unseen hardware.
- No publication provides a validated FAR/FRR for the current `[-5,0]` Oura optical calibration threshold.
