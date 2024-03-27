# win32yoink

A windows clipboard tool.

Based heavily on [win32yank](https://github.com/equalsraf/win32yank/tree/master), but adds the html format.

## Installation

### Download binary

```sh
wget https://github.com/haondt/win32yoink/releases/latest/download/win32yoink-windows-x86_64.zip
unzip win32yoink-windows-x86_64.zip
win32yoink.exe -o
```

### Install with cargo

```sh
cargo install win32yoink
win32yoink -o
```

### Install with Scoop

```sh
scoop install https://raw.githubusercontent.com/haondt/win32yoink/main/scoop/win32yoink.json
win32yoink -o
```

## Usage

Get the clipboard.

```sh
win32yoink -o
```

Set the clipboard

```sh
echo "hello brave new world" | win32yoink -i
echo "<p><b>hello</b> <u>brave</u> <i>new</i> world</p>" | win32yoink -i --html
```

If installed on powershell, can be accessed from wsl.

```sh
echo "text" | win32yoink.exe -i
```
