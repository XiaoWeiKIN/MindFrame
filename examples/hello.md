# Why MVCC exists

Databases need to let readers and writers make progress without forcing every operation through one global lock.

MVCC keeps multiple row versions so a transaction can read from a consistent snapshot while concurrent writers create newer versions.
