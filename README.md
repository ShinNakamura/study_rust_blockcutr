# 文字列の塊とセパレートするための文字列を受け取り、分割した結果の指定された位置に相当する部分をトリムして返す

セパレータに文字列を使うバージョンと正規表現が使えるバージョンを作る

## 注意

Rust 勉強中の練習課題です。

クローン、使用は自己責任でお願い致します。

## 仕様 文字列セパレータ

```html
<h1>header text</h2>
<p>aaa<br />
qqq</p>
<hr />
<br />
<br />
<h2>見出し1</h2>
<table>
    ...some contents #1
</table>
<hr />
<h2>見出し2</h2>
<table>
    ...some contents #2
    ...
</table>
<hr />
```

上記に対して

- セパレータ = `<hr />`
- ブロック = `1` (index は 0 スタート)

を指定すると、下記部分がトリムされた状態で返ってくる。

```html
<br />
<br />
<h2>見出し1</h2>
<table>
    ...some contents #1
</table>
```

## 仕様 正規表現セパレータ

文字列セパレータのサンプルで言えば、セパレータに

```
(?i)<hr[^>]*?>
```

のような正規表現が使える。

## 実行例

see `_test.sh`
```
bin=cutbystr
expect="aaa"
echo "aaa | oooo" | cargo run --bin $bin -- "|" 0
# aaa

bin=cutbyregex
expect="aaa"
echo " xxx <br />    aaa   <BR>oooo" | cargo run --bin $bin -- '(?i)<br[^>]*?>' 1
# aaa
```

