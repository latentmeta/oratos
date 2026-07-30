# Scoop manifest

Source of truth for the Scoop app manifest. Publish by copying to
[`latentmeta/scoop-bucket`](https://github.com/latentmeta/scoop-bucket) as `oratos.json`.

The `hash` field is the SHA-256 of the Windows zip from the GitHub Release `SHA256SUMS`.

```powershell
scoop bucket add latentmeta https://github.com/latentmeta/scoop-bucket
scoop install oratos
```
