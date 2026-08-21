# Glossary

This glossary defines the local terms used in `rd-engine`.

The repo is intentionally small, but the same ideas can be described in different ways across projects. This file keeps the core vocabulary explicit so people copying the engine know what each term means here and what can safely be renamed.

## RD Engine

The tiny reducer-governed deterministic state core in this repo.

What it is:
- a small reusable Rust core
- a pattern for keeping canonical state in buckets
- a way to mutate that state through reducers and emit receipts

What it is not:
- a full app framework
- one required memory schema
- one required game or UI model
- a claim of factual certainty, model knowledge, consciousness, or mind-like cognition

## Buckets

Live state containers holding the current canonical truth.

Here, `truth` means reducer-governed saved state: the data the host treats as
authoritative for continuity. It does not mean metaphysical truth or model-owned
knowledge.

What they are:
- the place lasting truth lives
- the data a host saves and reloads
- the state reducers are allowed to mutate

What they are not:
- one universal vocabulary
- just display labels
- freeform notes that bypass structure

## Reducer

The mutation boundary.

What it is:
- the part that approves or rejects requested state change
- the place where legal mutation rules live
- the reason state updates stay inspectable

What it is not:
- arbitrary freeform writing into state
- UI-driven state mutation by default
- prose-only reasoning with no canonical update rule

## Actions

Requests for change.

What they are:
- structured mutation requests
- inputs passed into reducers
- the trigger for state updates and emitted events

What they are not:
- guaranteed successful writes
- the state itself
- an excuse to skip reducer logic

## Events

Receipts describing what happened.

What they are:
- emitted records explaining successful or failed changes
- a readable explanation layer around mutation
- useful for logs, audit trails, and host rendering

What they are not:
- the canonical state itself
- a replacement for saving state
- prose that pretends to be the source of truth

## Canonical State

The durable truth you actually save.

What it is:
- the state object after reducer-approved mutation
- the thing continuity depends on
- the data that should survive reloads

What it is not:
- only a chat summary
- only a UI snapshot
- something reconstructed later from vibes

## Save Envelope

The saved representation of canonical state.

What it is:
- the persisted artifact for continuity and reload
- the direct save/load layer for the engine state
- a way to keep state durable without replaying prose

What it is not:
- transcript replay
- narrative memory pretending to be state
- a requirement that every host use one file format forever

## Templates

Optional canonical shapes or domain scaffolds.

What they are:
- a way to describe expected structure
- optional support for stronger consistency
- useful when the host wants predictable state shapes

What they are not:
- mandatory for every use
- the live buckets themselves
- the reducer boundary

## Host

The project that uses the engine.

What it is:
- the surrounding app, tool, runtime, or experiment
- the layer that decides how to render and use the state
- the place where local naming and domain logic can change

What it is not:
- forced to keep this repo's exact vocabulary
- locked to one product type
- required to expose every engine concept to end users

## Rename Early

This is not a mechanic, but it is an important local rule.

If `buckets`, `reducers`, `actions`, or `events` feel wrong in your project, rename them early. The mechanism matters more than the words.
