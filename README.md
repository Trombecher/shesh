# Shesh: The Next Level Shell

A shell that satisfies my own needs.

## How To Build

Just a regular `cargo build` will do.

Since this is a shell, it is faster to run directly in a terminal emulator. I have set up a PowerShell script
[`run.ps1`](./run.ps1), that will build and run the shell in a new terminal window. WINDOWS ONLY.

It may be necessary to adjust the Windows Terminal executable path in the script, depending on your installation.
For a fast development cycle, I recommend creating a new profile in Windows Terminal that runs the shell.

## Roadmap

* [ ] Syntax highlighting
* [ ] Command history
* [ ] Command auto-completion
* [ ] File auto-completion
* [ ] Filesystem search
* [ ] Sessions
* [ ] AI integration via Ollama
* [ ] Interpreter + Math
* [ ] Tool specific integrations
  * [ ] `cargo add [PACKAGE]` crate auto-completion
  * [ ] `bun add / install` package auto-completion
* [ ] Multiline input editing
* [ ] Windows set path variable (this is annoying to do with PowerShell)
* [ ] Constant expression evaluation
* [ ] Shortcuts for navigation