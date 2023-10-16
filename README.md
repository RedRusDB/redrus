## Reimplementing Redis with some extra features in Rust.


An attempt to write Redis(DBMS) with some extra feature in Rust
to learn about database internals.

Building a database from scratch has its own thrill, and you can leverage this to

build a database from scratch
learn database internals, starting with Redis
learn about advanced data structures, algorithms, and event loops
collaborate with other engineers and contribute back to Open Source

While our B+ tree now support concurrent operations, it's still a single
threaded database system, as our frontend (network/cli layer) doesn't support
handling concurrent requests yet.

_This is by no mean an idiomatic Rust implementation as I'm learning Rust
along the way._

-----

_Update (25/09/2022): The project is kind of on hold as I just started a new job and
still adapting to the new schedule I have. Hence, the progress will be slower.
Hopefully, I can regain my momentum after a couple of weeks._

----

## End Goal

The main focus to write a storage engine from scratch. This project now
includes it's own B+ Tree data structure, buffer pool, LRU replacement policy,
transaction manager, and lock manager.
