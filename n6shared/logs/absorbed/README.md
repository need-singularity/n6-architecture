# absorbed Log Archive Notes

## Migration Completed

- Migration date: 2026-04-12
- Source: 2,924 JSON files (approximately 11MB)
- Archive: `../absorbed_archive.tar.gz` (1.2MB)
- Index: `../absorbed_index.json` (includes filename, source path, grade, and n6 score)

## How to Restore

```bash
cd n6shared/logs
tar xzf absorbed_archive.tar.gz
```

## Notes

- Source files were deleted after being compressed into the archive
- DIGEST.md is an existing summary and is preserved as-is
- Individual files can be looked up through the index JSON
