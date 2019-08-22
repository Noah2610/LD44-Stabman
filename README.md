# LD44
Originally, our [Ludum Dare 44 jam entry][ludumdare].  

## Description
The theme for _Ludum Dare 44_ was...  
> Your life is currency

We went for a simple, generic approach:  
_You pay with your health to buy items._

The game is a _2D action-platformer_.  
Your goal is simply to get to the end of each level.  

Throughout the levels you will find _items_ which you can buy with your health as currency.  
Some items may be necessary to progress, others may just be _nice-to-have_.  

You regain health by defeating enemies.

Each level acts as a _checkpoint_; when you die, you restart at the beginning of the level.  
The game will also automatically save your progress to a savefile, each time you beat a level or die.  

When you beat the game, you may choose to start from the beginning again, keeping all your items ("NewGame+"-style).  
Timers for the current level and for the whole game will also appear  
once you start your second play-through on the same savefile.

## Controls
The exact controls are specified in the file `resources/config/bindings.ron`.  
You can change bindings in this file however you want; the syntax _should_ be self-explanatory enough.  
But careful: if you make a syntax error, the game will crash. Start the game from a console to see the error messages.

__Note:__  
For the controller buttons we are using the Xbox naming scheme.  
So the `A` button would be the `X` button on a DualShock controller, etc.

| Action                            | Keyboard                   | Controller                            | Notes                                                                      |
| :-------------------------------- | :------------------------- | :------------------------------------ | :------------------------------------------------------------------------- |
| Move LEFT                         | `A`                        | `DPadLeft`, `JoyStickLeft`            |                                                                            |
| Move RIGHT                        | `D`                        | `DPadRight`, `JoyStickRight`          |                                                                            |
| Jump                              | `Space`, `K`, `Up`         | `A`                                   | Hold down the jump button for a slower fall (lower gravity).               |
| Attack LEFT                       | `Left`, `H`                | `X`                                   |                                                                            |
| Attack RIGHT                      | `Right`, `L`               | `B`                                   |                                                                            |
| Buy item                          | `E`                        | `Y`                                   |                                                                            |
| Dash                              | `Shift`, `J`, `Down`       | `ShoulderButtons` (_left_ or _right_) | Need to hold down movement keys + press dash key; diagonal dashes work; can only dash in-air; __TODO:__ on controller, dashing only works when using the `DPad`. |
| Toggle pause                      | `P`                        | `Start`                               |                                                                            |
| Start game from main menu         | `Enter`, `Space`           | `A`                                   | Starts the _normal_ campaign, not _bonus_.                                 |
| Continue game from pause menu     | `Enter`, `Space`           | `A`                                   |                                                                            |
| Quit game from main menu          | `Escape`, `Q`, `Backspace` | `B`                                   |                                                                            |
| Quit to main menu from pause menu | `Escape`, `Q`, `Backspace` | `B`                                   |                                                                            |

## Campaign Types
There are two campaigns you can play:

- __normal__ campaign  
  The standard, recommended game.
- __bonus__ campaign  
  A collection of super hard levels, with the main focus on hardcore platforming.  
  This campaign type is primarily just meant for us to play; but if you're enjoying the game,  
  and are feeling confident enough, feel free to give this mode a try \>:)

## Post-Jam Version
After the jam ended at the end of April, we continued working on the game  
with the goal to release a finished product (like always).  
This time, we are proud to say, that we _actually finished this game_! (*__TODO:__ Not quite yet...*).

### Downloads
Post-jam-version binaries are also available on Google Drive.  
You can pick the latest version for your platform from this [Google Drive directory][bin-dir].  
(*__TODO:__ List latest versions here...*)

## Jam Version
To see the jam version, visit out our [Ludum Dare page][ludumdare], or  
check out the [`LD44-release`][LD44-release] tag.  

As always with our jam games, we never properly finish.  
This jam's game was _especially unfinished_.  
If you intend to check out our game, we recommend trying our post-jam version.

### Downloads
Jam-version binaries are available via Google Drive:

| Platform | Download |
|:--------:|:-------- |
| Windows  | [Google Drive][bin-jam-windows] |
| Linux    | [Google Drive][bin-jam-linux] |

---

## Development
### Tools Used
- __[Rust]__, programming language
- __[Amethyst]__, engine
- __[Deathframe]__, framework
- __[Vim]__, code editor
- __[GitHub]__, git repository hosting
- __[Gimp]__, tile/background/menu graphics
- __[Aseprite]__, sprite graphics
- __[Bosca Ceoil][BoscaCeoil]__, music
- __[FL Studio][FLStudio]__, music
- __[Tiled]__, level design
- __[Google Drive][GoogleDrive]__, file sharing
- __[Trello]__, task management
- __[Figma]__, stats menu design

---

## License
Distributed under the terms of the [MIT License][mit-license].

[ludumdare]:       https://ldjam.com/events/ludum-dare/44/veggietartar
[LD44-release]:    https://github.com/Noah2610/LD44/tree/LD44-release
[bin-jam-windows]: https://drive.google.com/file/d/1RtQ8vsJFH75WyZHTa1W_vxzchw6OhA7Y/view
[bin-jam-linux]:   https://drive.google.com/file/d/1xffSHQmDppZ4KGcbApzIFt4l_XvT2pX2/view
[bin-dir]:         https://drive.google.com/open?id=1XNMvBsOJkbbii6jp9T7Du9mNnso00Axn
[Rust]:            https://www.rust-lang.org/
[Amethyst]:        https://amethyst.rs/
[Deathframe]:      https://github.com/Noah2610/deathframe
[Vim]:             https://www.vim.org/
[GitHub]:          https://github.com/Noah2610/LD44
[Gimp]:            https://www.gimp.org/
[Aseprite]:        https://www.aseprite.org/
[BoscaCeoil]:      https://boscaceoil.net/
[FLStudio]:        https://www.image-line.com/flstudio
[Tiled]:           https://www.mapeditor.org/
[GoogleDrive]:     https://drive.google.com/
[Trello]:          https://trello.com/b/hkBWXYt9
[Figma]:           https://www.figma.com/
[mit-license]:     https://github.com/Noah2610/LD44/blob/master/LICENSE
