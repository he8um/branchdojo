# JSON Output Schema

`branchdojo check --path <path> --json` returns structured output for automation and future integrations.

## Top-Level Shape

```json
{
  "exercise": "conflict-basic",
  "status": "failed",
  "score": 4,
  "total": 6,
  "checks": [],
  "next_steps": []
}
```

## Fields

| Field | Type | Required | Description |
|---|---|---:|---|
| `exercise` | string | yes | Exercise ID from `.branchdojo.json`. |
| `status` | string | yes | `passed`, `warning`, or `failed`. |
| `score` | number | yes | Count of passed required checks. |
| `total` | number | yes | Count of required checks. |
| `checks` | array | yes | Individual check results. |
| `next_steps` | array | yes | Suggested next steps. |

## Check Shape

```json
{
  "id": "working_tree_clean",
  "label": "Working tree is clean",
  "status": "failed",
  "severity": "required",
  "message": "Uncommitted changes are present."
}
```

## Check Fields

| Field | Type | Required | Description |
|---|---|---:|---|
| `id` | string | yes | Stable machine-readable check ID. |
| `label` | string | yes | Human-readable label. |
| `status` | string | yes | `passed`, `warning`, or `failed`. |
| `severity` | string | yes | `required` or `warning`. |
| `message` | string | no | Optional detail. |

## Stability Rule

Check IDs should be stable once released. Labels can be improved, but IDs are used by automation.

## Error JSON

In v0.1, errors may remain human-readable on stderr. Structured error JSON can be added later.
