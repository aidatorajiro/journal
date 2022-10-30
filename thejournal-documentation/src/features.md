# Closer look at the app's features

In this chapter, I will introduce several main features of the app. After the player [^pl] opens the app, a window will pop up as the picture below.

![](top.png)  
Screenshot of the start page of the app

There are four buttons on the start page. After clicking these buttons, you will jump to another page.

## New Page: Writing a new note entry - 85% completed

After clicking "NewPage" button, the player will see a screen like below:

![](new.png)  
Screenshot of the "NewPage" page of the app

In this page, you can create a new note. Firstly, pressing "Add Texts" will yield a textbox in the right grey pane. 

![](hellow.png)

In this app, a note should be separated in smaller pieces (called "fragments") to allow rearrangement or recombination of notes thereafter. [^tx] To help this, if you type consecutive two newlines, it will produce another textbox below.

The future plan is allowing the player to split, merge, and tag fragments.

## Saving/Loading note data - 100% completed

After finish writing, you can press the "Save" button to save changes. Instantly, the app will save all the note data to "state.scn.ron" file, using Bevy's "scene" feature. At first it was very buggy, but now it's completely fine.

## Explore: Non-linear exploration of notes - 40% completed

future plan: tagging

## Linear: Linear exploration of notes  - 0% completed

![](apple-scr.png)  
Screenshot of Apple's Note App

## Migrate: Importing data from other services (e.g. Facebook) - 60% completed

future plan: GUI


[^pl]: I'll call users "players" from now on.

[^tx]: Each fragment corresponds to a textbox.