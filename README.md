```
    ____        __  __       ____  _      __  _
   / __ \____ _/ /_/ /_     / __ \(_)____/ /_(_)___  ____  ____ ________  __
  / /_/ / __ `/ __/ __ \   / / / / / ___/ __/ / __ \/ __ \/ __ `/ ___/ / / /
 / ____/ /_/ / /_/ / / /  / /_/ / / /__/ /_/ / /_/ / / / / /_/ / /  / /_/ /
/_/    \__,_/\__/_/ /_/  /_____/_/\___/\__/_/\____/_/ /_/\__,_/_/   \__, /
                                                                   /____/
```
---
PD ( Path Dictionary ) is a minimalist command-line tool that allow you to:
- save your filepath/dirpath in dictionary
- seek your path in dictionary
- list all paths in dictionary

## Features
- Easy to use
- Out-of-the-box access
- Flexible customizability
- No magic but powerful

## Install via curl command (Ubuntu)
```bash
# Fetch binary and put on eligible place
curl -L https://github.com/mtugb/path-dictionary/releases/latest/download/pd -o pd
chmod +x pd
mkdir -p ~/bin
mv -f pd ~/bin/
# If necessary, add one required line to ~/.bashrc
if ! grep -q '~/bin' ~/.bashrc; then
  echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc
  echo "Path added to .bashrc. Please restart your shell or run 'source ~/.bashrc'."
fi
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

# Look up the path you just subscribed
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
### 'tp' command usage
```
# Before this command, "work" must be subscribed ( pd set work [path] ).
tp work
```

## (Tip) You can also edit dictionary manually 
All of the dictionary data is saved in "~/.config/pd/pd.toml".
As this is a simple toml file, you can easily edit it directly if needed.
This can be helpful if you want to delete or add a series of paths.
### An example of pd.toml
```
[items]
c = "/mnt/c"
nvim = "/home/joseph/.config/nvim"
```

