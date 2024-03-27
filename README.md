# win32yoink

A windows clipboard tool.

Based heavily on [win32yank](https://github.com/equalsraf/win32yank/tree/master), but adds the html format.

Get the clipboard.

```bash
win32yoink -o
```

Set the clipboard

```bash
echo "hello brave new world" | win32yoink -i
echo "<p><b>hello</b> <u>brave</u> <i>new</i> world</p>" | win32yoink -i --html
```
