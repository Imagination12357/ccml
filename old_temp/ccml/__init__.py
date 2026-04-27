"""
CCML (Convenient Compatible Markup Language)

Usage:
    import ccml

    obj      = ccml.parse('name: "Alice"\nage: 30')
    json_str = ccml.to_json('name: "Alice"\nage: 30', indent=2)
"""

from .parser import parse, to_json

__all__ = ["parse", "to_json"]
__version__ = "0.1.0"