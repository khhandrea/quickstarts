# Flet
## Get started
```bash
# At project directory
pip3 install flet
flet create .
flet run
```

`flet run` options
- `--web`: web
- `-dr`: hot reload

## Architecture
- `Page`: main layout where the controls are arranged
- `Control` (a.k.a. widgets): should be added in `Page` class. some controls can have other controls 

## Todo App sample
- There are `main` function, `TodoApp` control class, and `Todo` control class which is an element of `TodoApp` list items

## References
- https://flet.dev/docs/getting-started/