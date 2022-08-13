# rust-crate-template

This is my rust template.

## Usage

- Install

```sh
brew install fd sd rename
```

- Use this tempalate

```sh
cd YOUR_WORKSPACE
git clone git@github.com:scepter914/rust-crate-template.git YOUR_PACKAGE_NAME
cd YOUR_PACKAGE_NAME

# case1. If you'd like to cleanup manually
./setup.fish
mv PACKAGE_README.md README.md
rm setup.fish
rm -rf .git # When you add this to an existing repository

# case2. If you'd like to cleanup automatically
./setup.fish --clean
```
