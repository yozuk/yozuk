# zuk

Yozuk command-line interface.

## Installation

```bash
cd yozuk/zuk
cargo install --path .
```

## Example

```bash
$ zuk generate 3 uuids
UUID Generator: Generating 3 UUIDs
d6d2950d-eb7f-4fea-a601-d99747e1dfcc
db7afcfc-d998-44dc-847e-e908a29a89f7
e4a13ac1-6c06-48ea-ab27-ef9d55cd11af
```

## HTTP Server Mode

```bash
zuk --mode http-server --server-addr 127.0.0.1:8080

curl --request POST 'http://127.0.0.1:8080/run' --form 'query.json="{\"tokens\":[{\"data\":\"uuid\"}]}"' | jq
{
  "type": "ok",
  "output": {
    "module": "UUID Generator",
    "sections": [
      {
        "data": "Generating 1 UUID",
        "media_type": "text/plain",
        "kind": "comment"
      },
      {
        "data": "d81d564e-89d3-4db1-8e41-57a23036b48f",
        "media_type": "text/plain",
        "kind": "value"
      }
    ]
  }
}
```
