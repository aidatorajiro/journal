# Thejournal - a note-taking app using Bevy

This is a non-linear note-taking app using Rust and Bevy.

# Introduction: What is this App?

I build an note-taking app that keeps my everyday thoughts. The application has a non-linear experience, allowing me to look back what I have written in past, and connecting different ideas in different time. Thus, it has features to help me not only keep note as a digital data, but also track modification and connect different pieces of existing note pages to create a new one. The app is written in Rust and uses a game development framework called Bevy. Bevy employs a peculiar data management called ECS.

## Project structure

- .cargo - build configuration
- .vscode - editor configuration
- assets - application assets
- dev-memos - dev diary
- src - source code
- thejournal-documentation - mdbook documentation (project report)
- backup.sh - backup the save data to google drive using restic and rclone
- generate_facebook.sh - automatically generate typedef from Facebook JSON backup files
- mdbook.sh - generate mdbook documentation (different from rustdoc documentation)
- rust-toolchain - specifies rust toolchain version
