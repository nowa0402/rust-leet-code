# Valid Palindrome

[link](https://leetcode.com/problems/valid-palindrome/)

フレーズが回文であるとは、すべての大文字を小文字に変換し、すべての英数字以外の文字を取り除いた後に、そのフレーズが前から読んでも後ろから読んでも同じになる場合を指します。英数字には文字と数字が含まれます。

文字列sが与えられたとき、それが回文であればtrueを、そうでなければfalseを返してください。

## 例1

- 入力: s = "A man, a plan, a canal: Panama"
- 出力: true
- 説明: "amanaplanacanalpanama"は回文です。

## 例2

- 入力: s = "race a car"
- 出力: false
- 説明: "raceacar"は回文ではありません。

## 例3

- 入力: s = " "
- 出力: true
- 説明: 非英数字を取り除いた後、sは空の文字列 "" になります。空の文字列は前から読んでも後ろから読んでも同じであるため、回文です。

## 制約

- 1 <= s.length <= 2 * 10^5
- sは印刷可能なASCII文字のみで構成されます。
