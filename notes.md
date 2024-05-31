## 2024-05-31 notes on user interfaces

last time I worked on this project (when I called it peanut script), I gave some thought to how the
user interface would be implemented. it's an impossible question to answer when the project isn't
well-defined and it's goals changes over time. when I frame the project as "just to have fun" and
"to exercise my brain", it makes sense to take multiple approaches, and see how difficult
supporting them is. there is some structure to this; tables of device form factor (mobile or
desktop) and platform (mac or windows or linux), but it also get's hairy; on linux: GTK, FLTK, QT,
or web? web is also supported on mac and windows, and although web is supported on mobile (ios and
android) often native app UIs will perform better.

the goal of tootvm is to make building "abstractions" ... "fun". or at least to explore that avenue
of design. there are UI concepts that are common among most platforms, like navigation between
"menus" or screens, handling interactions with buttons and other user input fields, and overal flow
and state of the application. these are things that are poorly defined by platforms and toolkits,
usually so they can be as flexible as possible, which is a void where this project might explore. I
think there are some commonalities and overlap between ios and android's UI toolkits, which I think
react native has bridged; look at that project can give us some ideas to get started. but whatever
is done within tootvm, it won't be tied to the platform like react native is; the idea is to
explore how the "abstraction" could help implement a complete app down the road. how the app will
communicate with the platform should be left open, and might be solved a number of ways, not just a
simple "plug and play" type of library.
