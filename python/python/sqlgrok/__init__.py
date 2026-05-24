"""Python test shim for sqlgrok.

This package intentionally exposes a small SQLGlot-shaped surface first so
SQLGlot's own test helpers can be adapted against the Rust implementation.
"""

from ._native import transpile

__all__ = ["transpile"]
