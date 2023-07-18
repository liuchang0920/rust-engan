
# Install instructions

* https://github.com/jepsen-io/maelstrom/blob/main/doc/01-getting-ready/index.md
*  `wget https://github.com/jepsen-io/maelstrom/releases/download/v0.2.3/maelstrom.tar.bz2``
* `tar -xf maelstrom.tar.bz2``

Command line to run

* task 1: `./maelstrom test -w echo --bin ../rust-engan/russt_engan/target/debug/echo  --time-limit 5`
* task 2: `./maelstrom test -w unique-ids  --bin ../rust-engan/rust_engan/target/debug/unique_ids  --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition`
* task 3: `./maelstrom test -w broadcast --bin ../rust-engan/rust_engan/target/debug/broadcast --node-count 5 --time-limit 20 --rate 10 --nemesis partition`

sample output

```text
...
Everything looks good! ヽ(‘ー`)ノ
```