## Contribution Principles

All contributions must follow these three principles:

1. **Convenient to humans**
2. **Compatible with JSON**
3. **Clear to parsers**

Among these, **JSON compatibility is a strict requirement and must never be compromised**.
Any JSON document must be representable in CCML, and conversely, anything that cannot be represented in JSON must also be impossible to represent in CCML.

The first and third principles are often in tension. A contribution may slightly reduce one of them if it significantly improves the other. However, such trade-offs are only acceptable when the **overall balance between human convenience and parser clarity is improved**.

Ideally, a contribution should improve both principles. In all cases, neither principle should fall below a reasonable baseline.