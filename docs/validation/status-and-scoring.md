# Status and Scoring

## Status Values

- `PASSED`
- `WARNING`
- `FAILED`

## Status Calculation

```text
if any required check failed:
    status = FAILED
else if any warning check produced warning:
    status = WARNING
else:
    status = PASSED
```

## Score Calculation

v0.1 uses simple required-check scoring:

```text
score = number of passed required checks
total = number of required checks
```

Warnings do not reduce score.

## Why Not 100-Point Score?

A 100-point score suggests precision that v0.1 does not need. Exercise validation is about whether the final state is correct. `5/6` is clearer than `83/100`.

## Output Examples

```text
Status: PASSED
Score: 8/8
```

```text
Status: WARNING
Score: 8/8
```

```text
Status: FAILED
Score: 5/8
```

## Future Scoring

v0.3 may introduce weighted scoring or report generation, but v0.1 should remain simple.
