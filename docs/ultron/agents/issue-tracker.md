# Issue tracker: Local Markdown

Issues for this repo live as markdown files next to the task that produced them (ADR-0006, item 7). The file is the source of the issue's state.

## Conventions

- Each task has its folder: `tasks/NNNN-slug/` under the Ultron tree, with `task.md` and, when there is one, `prd.md`
- An issue of that task is `tasks/NNNN-slug/issues/<NN>-<slug>.md`, numbered from `01` per task
- Its reference is `NNNN-slug#NN`; `#NN` alone works while the number is unique across tasks
- The state is a `status:` line near the top (`open` or `closed`); triage roles use the strings in `triage-labels.md`
- Comments and the closing note append to the bottom
- There are no milestones: the task is the only level of work (ADR-0006, item 10)

## When a skill says "publish to the issue tracker"

```bash
python3 .claude/skills/ultron-tracker/scripts/create_issue.py --root . --task NNNN-slug --title "..." --body "..."
```

Without `--task`, nothing is created: the `task.md` itself is the issue.

## When a skill says "fetch the relevant ticket"

```bash
python3 .claude/skills/ultron-tracker/scripts/issue_client.py read NNNN-slug#NN --json
```
