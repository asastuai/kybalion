# arXiv cs.CY Endorsement Request — Strategy & Materials

## How arXiv endorsement works

To submit to arXiv cs.CY, you need an endorsement from someone who has submitted ≥3 papers to cs.CY (or related cs.* categories) more than 3 months ago and less than 5 years ago.

**Hard rules:**
- Send ONE email at a time. Wait 2 weeks before trying the next candidate.
- arXiv guidelines explicitly prohibit mass-emailing endorsers.
- Include identifier (ORCID, GitHub, scholarly profile) and a stable link to the paper.

## Before sending the first email

1. **Verify candidate's endorser eligibility.** Visit each candidate's recent arXiv abstract page and look at the bottom for "Which authors of this paper are endorsers?" Confirm the candidate is listed.
2. **Find their email.** It's on the abstract page just under the "Submission history" section.
3. **Decide on identifier.** ORCID iD is the gold standard — free, takes 2 minutes at <https://orcid.org/>. Without it, GitHub profile (github.com/asastuai) is acceptable.
4. **Stable URL for the paper.** The PDF must live at a stable URL the endorser can reach. Best option: push the regenerated PDF to GitHub Pages (in the kybalion repo) or to asastuai.github.io/opus/paper.pdf.

## Candidates (ranked by alignment, contact one at a time)

### 1. Mohamed Mabrok — "The Non-Optimality of Scientific Knowledge: Path Dependence, Lock-In, and The Local Minimum Trap"
- **Why this fits:** His work on path dependence and structural constraints in scientific knowledge is sympathetic to the framing we propose — that older intellectual structures can persist and rhyme with newer ones. Most likely to grasp a "framing contribution."
- **Action:** Find the paper on arxiv.org/list/cs.CY/recent, verify endorser status, get email.

### 2. Mirco Musolesi (with Giorgio Franceschelli) — "On the Creativity of AI Agents"
- **Why this fits:** Musolesi is a senior researcher with sustained cs.CY presence (likely meets eligibility easily). Work on creativity-as-computation is adjacent to our generative/formative framing.
- **Action:** Verify Musolesi's endorser status (he probably qualifies; Franceschelli may not yet).

### 3. Rui Chai — Wuxing Institutional Architecture series
- **Why this fits:** He cites Wuxing (Chinese five-element philosophy) in cs.CY. Someone willing to import Eastern philosophical structures into computer science is unusually open to interdisciplinary framings. Cultural alignment is high.
- **Action:** Verify endorser status (might be early-career and not yet eligible).

### 4. Salvatore Flavio Pileggi — "An ontological approach to foster the convergence... of frameworks for Trustworthy AI"
- **Why this fits:** Ontology work in CS tolerates conceptual papers. Less directly aligned but a reasonable fallback.

## Email template

> **Subject:** arXiv cs.CY endorsement request — interdisciplinary paper on hermetic philosophy and cryptography
>
> Dear Dr. [Last Name],
>
> I am an independent researcher in Buenos Aires writing to ask whether you would consider endorsing me for a single submission to arXiv's cs.CY (Computers and Society) category.
>
> The paper, *"Hermetic Computing: A Reading of Cryptographic and Computational Primitives Through the Seven Hermetic Principles,"* is an interpretive framework that maps the seven principles of the *Kybalion* (1908) onto established concepts in cryptography and quantum computing. Each principle is paired with a computational analogue and expressed as a Rust trait, with two illustrative artifacts (a hash function and a stream cipher) included as pedagogical demonstrations. I am explicit throughout that the contribution is framing and pedagogy, not cryptographic novelty: the artifacts use floating-point arithmetic, have not been formally cryptanalyzed, and are not proposed as production primitives. Rigorous purpose-bound encryption is acknowledged to live in Attribute-Based and Functional Encryption.
>
> I came across your paper *"[Title]"* while looking for recent cs.CY work that takes interdisciplinary framing seriously, and I thought your perspective on **[ONE SENTENCE about the specific resonance with their work]** might align with the spirit of this submission.
>
> The paper PDF: [STABLE URL]
> Source code: <https://github.com/asastuai/kybalion>
> Narrative context: <https://asastuai.github.io/opus/>
> Identifier: [ORCID URL or github.com/asastuai]
>
> I understand the endorsement process is selective and that no obligation attaches. If this isn't a fit, I would also welcome any suggestion of an alternative endorser whose work this might suit better. Thank you for your time.
>
> With appreciation,
> Juan Cruz Maisu
> Independent Researcher
> Buenos Aires, Argentina
> juancmaisu@outlook.com

## Per-candidate personalization (the [ONE SENTENCE])

- **Mabrok:** "your work on path dependence and the local-minimum trap in scientific knowledge"
- **Musolesi:** "your work on the creativity of AI agents and the philosophical questions it surfaces"
- **Chai:** "your willingness to draw on Wuxing institutional architecture in cs.CY work, which I read as a kindred openness to pre-modern philosophical structures"
- **Pileggi:** "your ontological framing of trustworthy AI, which suggests a tolerance for conceptual contributions in this category"

## After endorsement is granted

1. Create arXiv account (if not already).
2. Submit the paper through the arXiv portal, selecting cs.CY as primary category.
3. Optional secondary: cs.GL (general literature) or cs.OH (other) — only if helpful.
4. Wait for moderator review (1-3 days typical).
5. Once accepted, paper gets DOI-style identifier (arxiv.org/abs/YYMM.NNNNN). Update Opus and README to link to it.
