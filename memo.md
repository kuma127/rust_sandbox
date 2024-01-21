# 気になることメモ

## 言語仕様

- トレイト=インターフェース?
- クラスの変数やメソッドのアクセスは::（Wコロン）を使う
- スライス→配列を範囲を指し示す→参照型
- スライスの内部は「ファットポインタ」という構造になっている→参照先と要素数を持っているから
- &strは「文字列スライス」
- if-let文で条件式の結果を値として返却できる
- Vector→可変長配列
- 所謂参照渡しをしないと、所有権ごと代入先に渡り、代入元を再度呼び出そうとするとエラーになる