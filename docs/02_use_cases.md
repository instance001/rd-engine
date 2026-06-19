# Use Cases

The bucket names are not the product.

The reducer boundary is the product.

## Project Memory

- templates
  memory record shapes, trigger cards, routing packets
- buckets
  active memory, stale memory, review queue
- actions
  remember, mark_stale, attach_trigger

## Workflow Tool

- templates
  stage definitions, expected artifacts
- buckets
  pending, running, failed, completed
- actions
  start_stage, fail_stage, retry_stage, finish_stage

## Repo Orchestration

- templates
  project records, dependency links
- buckets
  active repos, archived repos, blockers
- actions
  register_repo, archive_repo, add_blocker, clear_blocker

## Scenario Or Game Runtime

- templates
  entities, regions, conditions
- buckets
  runtime state, inventory, discovered facts
- actions
  move, claim, resolve, advance
