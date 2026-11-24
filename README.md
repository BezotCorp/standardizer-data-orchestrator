# Standardizer Data Orchestrator

Coordinates Standardizer data generation.

Responsibilities:

- receive an exact `standardizer-data-ron` commit;
- validate the checked-out RON source;
- execute an exact version of `standardizer-generate-data`;
- generate provenance;
- publish generated data into `standardizer-data-bin`;
- serialize BIN publications so concurrent requests cannot overwrite each other.

The RON and BIN repositories remain data repositories.
