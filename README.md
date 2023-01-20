# sqlship

Generate diagrams from SQL files

## Usage

```bash
cargo run -- -i tables.sql | dot -Tsvg -o out.svg && inkview out.svg
cargo run -- -i tables.sql -g mocodo # Then paste the result on https://www.mocodo.net/
```
