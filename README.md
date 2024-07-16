nowasm_efmt
===========

An example of [nowasm] that runs the WebAssembly binary of [efmt].

[nowasm]: https://github.com/sile/nowasm
[efmt]: https://github.com/sile/efmt

```console
$ echo -e '-module(foo).foo()->\n1+2;\nfoo()->\nok.'
-module(foo).foo()->
1+2;
foo()->
ok.

$ echo -e '-module(foo).foo()->\n1+2;\nfoo()->\nok.' | cargo run /dev/stdin
-module(foo).


foo() ->
    1 + 2;
foo() ->
    ok.
```
