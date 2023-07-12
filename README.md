# Dictionary Care

Processes a dictionary in form a collection of words and the number of occurances as input, like

```
test 5
word 8
the 14
engineer 2
```

and a list of words, like

```
the
The
bee
can
fly
```

and outputs the updated dictionary to console, like

```
bee: 1
can: 1
engineer: 2
fly: 1
test: 5
the: 16
word: 8
```

## Usage

### Via cargo:
```cargo run -- <dictionary_file> <input_file>```

### Executable:
```./dictionary <dictionary_file> <input_file>```
