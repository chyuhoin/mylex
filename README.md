# mylex
使用rust语言开发的lex，可以读取.l文件并输出lex.yy.c文件。

### 使用方法
在安装了cargo的情况下，执行
```shell
cargo run -- xxx.l
```
程序将根据xxx.l文件创建lex.yy.c文件。

### 说明
由于实现上的原因，本lex的字符集仅包含全部的字母、数字、运算符、括号，以及换行符、回车符、制表符三个特殊字符，不包括@#$%等特殊字符

如果需要在正则表达式中书写空格，请使用\s替代，而不是打空格。例如“空格或者换行”应该是`[\s\n]`而不是`[ \n]`

根目录下已经附带了几个.l文件可供参考
