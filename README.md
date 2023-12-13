# napi-lib

## create project 
```sh
npm install --global yarn
npm install -g @napi-rs/cli
napi new
```

## test performance btree find 
```sh
yarn install
yarn build && node test/btree.js
```
tree find cost: 74.108ms

```sh
$ cargo test test_btree_find -- --nocapture
```
test time cost 259ms

## doc 
https://bo7zwj4hof.larksuite.com/docx/CfNIdQiuGoKP56xdrHAuihZ0sFd