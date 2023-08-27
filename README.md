# Archived AP server

An implementation of archived AP server.

* Serving static archived resources.
* Redirecting old resource URLs if archived resources are available.
* Return 410 Gone in other.

## Fetch migration data

```
$ cat input.json
{
    "static_base_url": "https://archivedon.mizunashi.work/static/",
    "accounts": [
        "@mizunashi_mana@mstdn.mizunashi.work"
    ]
}
$ archivedon-fetch --input input.json --output output --fetch-outbox
$ ls output
map/  static/  webfinger/
```

## Serve

```
$ archivedon --port 8000 --resource-dir output
```
