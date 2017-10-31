
## simple

This is a simple (but not the simplest possible) dat archive: a single short
README file, which is added in one version and modified in the second (to
include a self-reference to the dat address).

## alphabet

This dat archive is intended to match the content tree examples from the
`hypercore` registry.

To generate single-character files with no trailing newline:

    echo -n 'a' > a
    echo -n 'b' > b
    echo -n 'c' > c
    echo -n 'd' > d
    echo -n 'e' > e
    echo -n 'f' > f

## tree

This archive has a bunch of deeply nested folders, to demonstrate hyperdrive
lookup behavior.
