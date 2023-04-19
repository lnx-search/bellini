# Bellini

*NOTE: Bellini requires nightly due to some internal compiler errors*

A document (de)serialization system built on top of `rkyv` for high efficiency doc storage.

Bellini's primary features are fast deserialization, we trade off disk usage in order to support rkyv's
zero-copy system which is more useful for lnx (and probably yourself, compression fixes this issue.)

