# Merge two sorted lists

2つのソートされたリンクリスト list1 と list2 の先頭が与えられたとき、2つのリストを1つのソートされたリストにマージしてください。

このリストは、最初の2つのリストのノードをつなぎ合わせることで作成されるべきです。

マージされたリンクリストの先頭を返してください。

## 例1

- 入力: list1 = [1,2,4], list2 = [1,3,4]
- 出力: [1,1,2,3,4,4]

## 例 2

- 入力: list1 = [], list2 = []
- 出力: []

## 例 3

- 入力: list1 = [], list2 = [0]
- 出力: [0]

## 制約

- 両方のリストのノード数は [0, 50] の範囲です。
- Node.val は -100 以上 100 以下の整数です。
- 両方の list1 と list2 は非減少順にソートされています。
