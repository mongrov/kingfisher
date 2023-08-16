# KingFisher (kf)

Fast command line tool to run template engines

## Usage

Use handlebar template to produce output.

```sh
kf hb input.yaml handlebar.tmpl output.file
```

## Dev Workflow

```bash
cargo bump patch --git-tag
git push --follow-tags
```

```bash
# cargo release
cargo publish

# build a release file
cargo build --release
```
