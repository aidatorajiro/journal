# Dev Diary: How I made this app?

I'll explain some of the key concepts that I encountered during the development.

## Bevy and ECS

The biggest obstacle I encountered during the development is Bevy's ECS mechanism. ECS stands for Entity, Component, and System.

### Entity

In Bevy, unlike other application frameworks like React, there are no hierarchal data structures. Instead, every objects (called `Entity`) are placed on a unique, universal, and global set of objects. Then, how can we store different types of data if there's only one big set?

### Component

To store various kinds of data, programmers can "tag" each entity with a custom data structure. This is called `Component`. Also, `Component` is used for "built-in" bevy features such as UI, 3d rendering, interaction, and so on. Although `Entity` itself is just an 64-bit identification number and doesn't have any other data, `Component` can be registered to Bevy together with the `Entity`. How a Bevy program access `Entity` and `Component`? This is related to *parallelism*, which is one of the biggest advantage of Bevy.

### System

`System` is a special type of Rust functions that can be registered into Bevy's App. As Bevy doesn't have hierarchy mechanism, any system can access any `Entity` and `Component`. There is no access restrictions. However, `System` must declare which type of data it wishes to read or write, in its arguments field. For example, the code below is the argument field of `System` that watches whether the player clicks the buttons, and initiates a jump to another page if clicked.

```rust
fn top_button_update_system(
    mut interaction_query: Query<
        (&Interaction, &TopPageButton, &mut UiColor),
        Changed<Interaction>,
    >,
    mut ev_newpage: EventWriter<JumpToNewPage>,
    mut ev_explore: EventWriter<JumpToExplore>,
    mut ev_linear: EventWriter<JumpToLinear>,
    mut ev_migrate: EventWriter<JumpToMigrate>
)
```
