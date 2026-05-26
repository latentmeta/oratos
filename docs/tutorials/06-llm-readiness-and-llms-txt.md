# 06: LLM readiness and llms.txt

## Mental model

“LLM readiness” in v0.1.0 means: “Does this page have enough structured, extractable signal that a summarizer can avoid guessing?”

## Why it matters

LLM-based agents often rely on shallow extraction: titles, descriptions, headings, and obvious summaries. Missing signals increase hallucination risk.

## How Oratos models it

Oratos includes lightweight LLM readiness checks for:

- missing `llms.txt`
- weak or missing title/description
- missing primary heading
- important images lacking alt/captions
- little extractable text
- no clear summary candidate

It can also generate a conservative `llms.txt` draft from discovered pages.

## Implementation notes

Oratos v0.1.0 does not call LLM APIs. It generates deterministic prompts and artifacts only.

## Tests

`oratos-generate` includes unit tests that assert llms.txt output includes expected sections.

## Limitations

`llms.txt` is an emerging convention. It should supplement (not replace) good HTML, metadata, sitemaps, and structured data.

## Future improvements

Optional provider integrations and richer summaries (still keeping deterministic defaults).

