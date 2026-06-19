# Drop-In Guide

This repo is meant to be easy to steal from.

## Smallest Useful Use

You need only four concepts:

- `EngineState`
- `EngineAction`
- `Reducer`
- `apply_action(...)`

Everything else is optional support.

If you are in a hurry, copy `engine.rs`, make one reducer, and ignore the rest until you need save/load.

## Typical Integration

1. Copy the crate or the core files into your project.
2. Rename buckets to match your domain.
3. Write one reducer for the mutations you care about.
4. Persist `EngineState` with the save helpers if you need continuity.

## Rename Early

If these words feel wrong, change them on day one:

- `buckets`
- `actions`
- `events`
- `reducer`

The mechanism survives renaming.
Clarity in your own codebase matters more than fidelity to this repo's wording.

## Good Adaptation Signs

You are using the pattern well if:

- the lasting truth is inspectable in one state object
- every important mutation goes through a reducer
- you can explain state changes by reading emitted events
- you can save and reload without replaying prose

## First Real Example To Build

A good first adaptation is one of:

- a todo/task list
- a project memory store
- a workflow stage tracker

Those are small enough to understand in one sitting and large enough to prove the pattern.
