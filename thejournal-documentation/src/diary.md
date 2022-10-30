# Dev Diary: How I made this app?

## Font

The development of the app was full of constantly changing plans and technical obstacles. As it is note-taking app, I first thought of introducing fancy color fonts such as COLRv1. However, after playing with Rust libraries like [swash](https://github.com/dfrg/swash), I learned using COLRv1 in Rust is difficult, so I abandoned the idea.

## Bevy and ECS

The biggest obstacle I encountered during the development is Bevy's ECS mechanism. ECS stands for Entity, Component, and System.

### Entity

In Bevy, unlike other application frameworks like React, there are no hierarchal data structures. Instead, every objects (called `Entity`) are placed on a unique, universal, and global set of objects. Then, how can we store different types of data if there's only one big set?

### Component

To store various kinds of data, programmers can "tag" each entity with a custom data structure. This is called `Component`. Also, `Component` is used for "built-in" bevy features such as UI, 3d rendering, interaction, and so on. Although `Entity` itself is just an 64-bit identification number and doesn't have any data, `Component` can be registered to Bevy together with the `Entity`. How a Bevy program access `Entity` and `Component`?

### System

`System` is a special type of Rust functions that can be registered 

### Resource

### Parent-Children

### Race Condition

### Scene

## Egui

## Directed Graph