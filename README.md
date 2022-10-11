# dnote-rs
dnote clone in rust


```
>> dnote --help
Dnote - a simple command line notebook

Usage:
  dnote [command]

Available Commands:
  add         Add a new note
  edit        Edit a note or a book
  find        Find notes by keywords
  help        Help about any command
  login       Login to dnote server
  logout      Logout from the server
  remove      Remove a note or a book
  sync        Sync data with the server
  version     Print the version number of Dnote
  view        List books, notes or view a content

Flags:
  -h, --help   help for dnote

Use "dnote [command] --help" for more information about a command.
```

```
>> dnote add --help
Add a new note

Usage:
  dnote add <book> [flags]

Aliases:
  add, a, n, new

Examples:

 * Open an editor to write content
 dnote add git

 * Skip the editor by providing content directly
 dnote add git -c "time is a part of the commit hash"

Flags:
  -c, --content string   The new content for the note
  -h, --help             help for add
```