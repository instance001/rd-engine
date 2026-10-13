# Provenance: RD Engine

## Project identity

- Canonical repository: <https://github.com/instance001/rd-engine>
- Public originating identity: `instance001` (verified from the canonical repository history).
- Status: `IMPLEMENTED` — a Rust crate and examples are present; this is not an empirical claim about downstream outcomes.
- Verified origin record: initial repository commit [`542c1f8`](https://github.com/instance001/rd-engine/commit/542c1f8a0c54084e402158be24c452d157b7954b), dated 2026-06-20. This is a Git history record, not a claim of first publication or first invention.

## Intellectual contribution

RD Engine documents and implements a small, usage-agnostic state core in which durable host state is held in buckets, changes are proposed as actions, reducers own permitted mutation, outcomes emit receipts, and canonical state can be saved directly. Its claimed contribution is this documented combination and its small, copyable Rust implementation — not reducers, state machines, deterministic execution, persistence, or event logs in general.

## Prior art and influences

The repository deliberately uses established concepts including reducers, state management, event logging, serialization, and deterministic execution. It makes no claim to have invented those concepts. No exhaustive prior-art survey or exclusive-priority claim has been verified for this record.

## Implementation relationships

The README identifies `semantic-signal-alphabet`, `llm-defined-persistent-memory`, and `chatty-quest` as adjacent work. Those references document conceptual relevance; this record does not assert code dependency, derivation, or endorsement without project-specific evidence.

## Attribution and citation

### If you copy RD Engine code

Keep applicable AGPLv3-or-later licence information and SPDX identifiers with copied source, retain existing legal notices, and mark material modifications appropriately. See `LICENSE` and `NOTICE.md`.

### If you fork or adapt its implementation

Follow the applicable licence for copied expression. A suggested factual acknowledgement is: “Includes/adapts code from RD Engine, <https://github.com/instance001/rd-engine>.” This wording is recommended, not a promotional endorsement.

### If you independently implement a related method

An independent implementation of an abstract method is not automatically subject to RD Engine’s software licence. Where the documented architecture genuinely informed the work, voluntary acknowledgement is encouraged: “Methodological inspiration: RD Engine’s documented reducer-governed state architecture.” Cite this repository if useful.

### Academic discussion

Use `CITATION.cff` for a repository citation. Do not imply that use or citation establishes validation, endorsement, ownership of general concepts, or exclusive priority.

## Licensing

The crate metadata states `AGPL-3.0-or-later`; this record aligns public wording to that declaration and does not alter the official licence text. The licence covers protected expression, not independent creation of ideas or methods. No new AGPL §7 additional term is imposed. The RD Engine name and logo are not licensed by this file; truthful non-endorsing reference is recommended, and logo use is voluntary pending any separate trademark policy and rights review.

## Evidence and limitations

Evidence: `README.md`, `Cargo.toml`, `LICENSE`, source files, and canonical Git history. The repository’s copyright-holder chain, third-party contribution status, trademark status, and exhaustive prior art are not established here and require human/legal review before any new binding term is adopted.
