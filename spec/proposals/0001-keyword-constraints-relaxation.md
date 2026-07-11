# CCP-0001: keyword constraints relaxation

<!-- Not all sections are mandatory; complete only what's relevant to your proposal. -->

## Metadata

| Item | Value |
|------|-------|
| Status | Draft |
| Author | imagination12357 |
| Created | 2026-07-11 |
| Target Version | TBD |

---

## Summary

Boolean values and null are represented as keyword literals, but their exact spelling varies slightly across programming languages. As a result, most people are accustomed to the conventions of the languages they use. However, CCML's current policy strictly accepts only `true`, `false`, and `null`. This proposal suggests relaxing that restriction by recognizing a wider range of commonly used keyword variants while mapping them to the canonical values `true`, `false`, and `null`.


---

## Specification

### Before

```ccml
a: false
b: false
c: false

d: true
e: true
f: true

g: null
h: null
i: null
j: null 
k: null
l: null
m: null
n: null
o: null
```

### After

```ccml
a: false
b: False
c: FALSE

d: true
e: True
f: TRUE

g: null
h: nil
i: none
j: Null 
k: Nil
l: None
m: NULL
n: NIL
o: NONE
```

---

## 3C Evaluation

### Compatible to JSON

- [x] Compatible
- [ ] Breaking

Reason:

### Clear to Parser

- [ ] -2: Greatly increases parser complexity
- [ ] -1: Slightly increases parser complexity
- [x]  0: No significant change
- [ ] +1: Slightly reduces parser complexity
- [ ] +2: Greatly reduces parser complexity

Reason:

### Convenient for Humans

- [ ] -2: Very inconvenient
- [ ] -1: Slightly inconvenient
- [ ]  0: No significant change
- [x] +1: Slightly more convenient
- [ ] +2: Much more convenient

Reason:

---

## Alternatives Considered

<!-- Alternative approaches considered -->

## Drawbacks

Allowing multiple keyword variants for the same value may reduce consistency across documents. Different authors may prefer different spellings, resulting in a mixture of styles within the same codebase and making documents appear less uniform.
