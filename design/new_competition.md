(This document is not meant to be a documentation, but more a help for the developer to keep an overview and to plan the behavior.)

# General Stuff

It should be possible to go back to a previous screen and most of the information (if possible) should be retained when going forward again.

# 1. Stage: Base Information

Idea: Enter basic information (see below) and add groups as well as the teams in the groups.

The menu to add groups and teams works in the way that the user can click a button integrated in the UI to add a new group/team. The group/team gets a generic basic name, but the text can be edited so that the user can enter the required names.

(take a look in the design.pdf for a exact design of the GUI)

|                 Information                 |    Datatype     | integrated |
| :-----------------------------------------: | :-------------: | :--------: |
|              Competition Name               |    `String`     |     ⬜️      |
|                 Date & Time                 | `chrono::Local` |     ⬜️      |
|                  Location                   |    `String`     |     ⬜️      |
|                  Executor                   |    `String`     |     ⬜️      |
|                  Organizer                  |    `String`     |     ⬜️      |
|                   Referee                   |    `String`     |     ⬜️      |
|             Competition Manager             |    `String`     |     ⬜️      |
|                  Secretary                  |    `String`     |     ⬜️      |
| Additional Text (i.e. mostly for greetings) |    `String`     |     ⬜️      |

# 2. Stage: Player Names

The user can enter up to 6 player names for each team. No names are required.
When going back to to stage 1 and forward to stage 2 again, most player names should be retained in order to minimize the annoyance and irritation of the user.

# (Probably) Required Widgets

https://docs.gtk.org/gtk4/class.SpinButton.html (vertical)

https://docs.gtk.org/gtk4/class.Calendar.html

https://docs.gtk.org/gtk4/class.Notebook.html

