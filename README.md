## mockbob

cOmMaNdLiNe uTiLiTy tO MoCk sEnTeNcEs lIkE ThIs.

### Usage

Please run `mockbob --help` for a list of all available options.

#### mock something

```shell script
mockbob hello world
```

#### read from stdin

```shell script
echo "hello world" | mockbob
```

#### use a different mocking strategy

```shell script
# this strategy would change every 3. char to uppercase
mockbob --strategy nth_char --nth 3 hello world

# randomly decide if th char should be uppercase
mockbob --strategy random hello world
```

#### ignore some characters

```shell script
# the characters a b c d and e will always be lowercase
mockbob --blacklist a b c d e -- hello world
```

#### clipboard

**only available if mockbob was compiled with the `clipboard` feature enabled`

```shell script
# mocks the clipboard contents and copies the result
mockbob --from-clipboard --to-clipboard
```

### Building

#### without clipboard support

```shell script
git clone https://github.com/j-brn/mockbob-cli.git
cd mockbob-cli
cargo build --release
# optional
strip target/release/mockbob
```

#### with clipboard support

**If you are running GNU/Linux with X11, please make sure to have `xorg` and/or `xorg-dev` installed.

```shell script
git clone https://github.com/j-brn/mockbob-cli.git
cd mockbob-cli
cargo build --release --features clipboard
# optional
strip target/release/mockbob
```