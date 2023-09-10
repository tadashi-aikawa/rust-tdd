# rust-tdd

[テスト駆動開発]をRustで実践するリポジトリ。

[テスト駆動開発]: https://shop.ohmsha.co.jp/shopdetail/000000004967/

## 使い方

ソースコードが変更されるたびにテストが走るように以下のコマンドを実行しておく。

```
cargo watch -x test
```

> **Note**
> 事前に[cargo-watch]のインストールが必要

[cargo-watch]: https://github.com/watchexec/cargo-watch

あとはひたすらコードを書いてリファクタリングしていくだけ。どの章の変更点かあとで確認しやすいよう、少なくとも章で1回はコミットし、prefixに章名を入れる。

`例`

```
[2章] Dollarの副作用除去
```

## Appendix

### [テスト駆動開発]の個人的な学習メモ

> [📰テスト駆動開発 読書メモ \- Minerva](https://minerva.mamansoft.net/Notes/%F0%9F%93%B0%E3%83%86%E3%82%B9%E3%83%88%E9%A7%86%E5%8B%95%E9%96%8B%E7%99%BA+%E8%AA%AD%E6%9B%B8%E3%83%A1%E3%83%A2)
