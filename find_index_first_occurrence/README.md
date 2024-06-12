# Find the Index of the First Occurrence in a String

以下の二つの文字列 needle と haystack が与えられたとき、haystack 内で needle が最初に出現する位置のインデックスを返してください。

もし needle が haystack の一部でない場合は、-1 を返してください。

## 例1

- 入力: haystack = "sadbutsad", needle = "sad"
- 出力: 0
- 説明: "sad" はインデックス 0 と 6 に出現します。最初の出現はインデックス 0 なので、0 を返します。

## 例2

- 入力: haystack = "leetcode", needle = "leeto"
- 出力: -1
- 説明: "leeto" は "leetcode" に出現しないので、-1 を返します。

## 制約

- 1 <= haystack.length, needle.length <= 10^4
- haystack と needle は小文字の英字のみから成ります。
