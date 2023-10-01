# Forked Diffy

See [diffy](https://github.com/bmwill/diffy) for docs on the version of `diffy` that is on crates.io.

This fork loosens some of the sanity checks and assertions validating that hunk headers match hunk content. In most use cases, those checks should probably still be run. However, I've been experimenting with ways for LLMs to edit flat files and have found that in many cases the model can generate accurate hunk content for unified diffs but practically never gets the hunk headers correct. `diffy` will fail parsing the Patch, but still successfully apply the Patch if the hunk header check is removed.


It also adds some `tracing` lines to help with debug in upstream Carabiner tech applications.