# Dir Traverse

Easy directory traversal. Two 

# Installation
`cargo install --path .`
# Usage
## Commands
### Traverse up
```bash
dir-traverse u {PATH}
```

Traverses up the directory tree. For example if you are in `/apple/orange/mango` and execute
`dir-traverse u ora` it will output `/apple/orange`


### Traverse down
```bash
dir-traverse d {PATH}
```

Traverses down the directory tree. If you are in `/apple` and type `man` (unless there are 
conflicting directories), you will be sent to `/apple/orange/mango`. Uses a breadth-first-search.

## Zsh
```bash
function u () {
  folder=$(dir-traverse u $argv[1])
  cd $folder
}

function d() { 
  folder=$(dir-traverse d $argv[1,-1])
  cd $folder
}
```
