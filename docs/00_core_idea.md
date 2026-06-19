# Core Idea

`RD Engine` is a tiny pattern:

1. keep canonical state in buckets
2. mutate those buckets only through reducers
3. emit events describing what changed
4. save the canonical state, not just prose about it

That is the whole point.

Everything else is adapter detail.

## Mental Model

- templates are optional canonical shapes
- buckets are the live state
- actions request change
- reducers approve or reject change
- events explain what happened
- hosts decide how to render it

## Why It Exists

A lot of projects become hard to trust once state is scattered across:

- UI-only flags
- improvised text summaries
- chat history
- side effects without receipts

This pattern keeps one boring truth source and one boring mutation boundary.
