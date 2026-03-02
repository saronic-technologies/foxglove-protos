# Foxglove-Protos
Generate rust bindings for foxglove protobufs.

## How to update schemas
- Fetch the latest schemas:
```
git submodule update --remote schemas
```
Note: If it's your first time cloning this repo, run `git submodule update --init --remote schemas`

- Copy the latest definitions to the proto directory:
```
cp schemas/schemas/proto/foxglove/*.proto proto/foxglove/
```
