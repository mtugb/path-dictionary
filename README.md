```
    ____        __  __       ____  _      __  _
   / __ \____ _/ /_/ /_     / __ \(_)____/ /_(_)___  ____  ____ ________  __
  / /_/ / __ `/ __/ __ \   / / / / / ___/ __/ / __ \/ __ \/ __ `/ ___/ / / /
 / ____/ /_/ / /_/ / / /  / /_/ / / /__/ /_/ / /_/ / / / / /_/ / /  / /_/ /
/_/    \__,_/\__/_/ /_/  /_____/_/\___/\__/_/\____/_/ /_/\__,_/_/   \__, /
                                                                   /____/
```
# Path Dictionary
---
PD ( Path Disctionary ) is a command-line tool that allow you to:
- save your filepath/dirpath in dictionary
- seek your path in dictionary
- list all paths in dictionary

## Features
- Easy to use
- Out-of-the-box access
- Flexible customizability

## Install via curl command (Ubuntu)
```bash
curl -L https://github.com/mtugb/path-dictionary/releases/latest/download/pd -o pd
chmod +x pd
mkdir -p ~/bin
mv pd ~/bin/
echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Usage
```plain_text
$HOME (current location)
├── projects/
│   └── my-app/
├── documents/
│   └── university/
│       └── ai-major/
└── music/
    └── guitar-tabs/
```

```bash
# Set path to the dictionary
pd set my-app ./projects/my-app

# Look up the path you just subsceibed
pd get my-app #output: /home/user/project/my-app

# List dictionary
pd list #output: [my-app] /home/user/project/my-app

cd ./documents/university/ai-major
pd set study .
pd get study #output: /home/user/documents/university/ai-major

# Search in dictionary
pd list | grep stu
#output: [**stu**dy] /home/user/documents/university/ai-major
```

## (Customize) Setup 'tp' command for teleporting
Add this function to your `~/.bashrc` to jump to saved paths:

```bash
tp() {
    local target=$(pd get "$1")
    if [ -n "$target" ] && [ -d "$target" ]; then
        cd "$target"
    else
        echo "Error: Directory not found or not registered."
    fi
}
```
### tp command usage
```
# Before this command, "work" must be subscribed ( pd set work [path] ).
tp work
```




