# WORD COUNTER

this is my own version of wc for personal use

### Usage

word count
```bash
wcc "hello world"
```

```bash
wcc "$(ls)"
```

disable echo and label
```bash
wcc -e "hello world"
```

line count
```bash
wcc -l "hello\nworld"
```

```bash
wcc -l "$(ls)"
```

```bash
wcc -l "hello
world"
```

### Test

```bash
cargo test
```

```bash
cargo test test_normal_run
```

```bash
cargo test test_line_run
```
