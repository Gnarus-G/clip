# clip
Copy the content of a file to the system clipboard.

## Install
```sh
export VER=$(wget -qO- https://github.com/Gnarus-G/clip/releases/latest | grep -oP 'v\d+\.\d+\.\d+' | tail -n 1);
curl -L https://github.com/Gnarus-G/clip/releases/download/$VER/clip-$OSTYPE.tar.gz -o clip.tar.gz
tar -xzvf clip.tar.gz clip
# Allow to able to run it from anywhere [Optional]
sudo mv clip /usr/local/bin
```

## Usage
```sh
echo text content | ./clip
```
