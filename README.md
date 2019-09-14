# LD44
Originally, our [Ludum Dare 44 jam entry][ludumdare].  

## Table of Contents
- [Description](#description)
- [Controls](#controls)
- [Campaign Types](#campaign-types)
- [Changing the window resolution](#changing-the-window-resolution)
- [Post-Jam Version](#post-jam-version)
  - [Post-Jam Version Downloads](#post-jam-version-downloads)
- [Jam Version](#jam-version)
  - [Jam Version Downloads](#jam-version-downloads)
- [Development](#development)
  - [Tools Used](#tools-used)
- [Licensing](#licensing)
  - [Fonts](#fonts)
    - [Undefined Medium](#undefined-medium)
    - [Ruji's Handwriting](#rujis-handwriting)
  - [SDL2](#sdl2)

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

Most of what's written in this README is directed at the _post-jam_ version.

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
There are three campaigns you can play:

- __normal__ campaign:  
  The standard, recommended game.  
  Includes __10 levels__; 3 in the _overworld theme_, and 7 in the _graveyard theme_.
- __bonus\_a__ and __bonus\_b__ campaigns:  
  A collection of super hard levels, with the main focus on hardcore platforming.  
  These campaign types are primarily just meant for us to play; but if you're enjoying the game,  
  and are feeling confident enough, feel free to give these a try __>:)__  
  Each of the bonus campaigns includes __3 levels__.

## Items
<style>
img#items-spritesheet {
  width: 600px;
  /* https://stackoverflow.com/a/24869226/10927893 */
  image-rendering: optimizeSpeed;             /* PREFER SPEED OVER SMOOTHING    */
  image-rendering: -moz-crisp-edges;          /* Firefox                        */
  image-rendering: -o-crisp-edges;            /* Opera                          */
  image-rendering: -webkit-optimize-contrast; /* Chrome (and eventually Safari) */
  image-rendering: optimize-contrast;         /* CSS3 Proposed                  */
  -ms-interpolation-mode: nearest-neighbor;   /* IE8+                           */
}

#items-box {
  display: grid;
  grid-template-columns: 60px 60px 60px 60px 60px 60px 60px 60px 60px 60px;
}

#items-box > .item-name {
  font-size: 11px;
  font-weight: bold;
  text-align: center;
}

#items-box > .item-name.item-unused {
  font-size: 10px;
  font-weight: normal;
  font-style: italic;
}
</style>

<div id="items-container">
  <img
   id="items-spritesheet"
   src="./resources/spritesheets/items.png"
   alt="Item Icons">
  </img>

  <div id="items-box">
    <div class="item-name">
      Extra Jump
    </div>
    <div class="item-name">
      Wall Jump
    </div>
    <div class="item-name item-unused">
      (unused)
    </div>
    <div class="item-name">
      Bullet Deflect
    </div>
    <div class="item-name">
      Damage Up
    </div>
    <div class="item-name">
      Speed Up
    </div>
    <div class="item-name">
      Jump Height Up
    </div>
    <div class="item-name">
      Bullet Shoot
    </div>
    <div class="item-name">
      Dash
    </div>
    <div class="item-name">
      Knockback Up
    </div>
  </div>
</div>

- __Extra Jump__ _Stackable_  
  Each pickup grants an extra jump in mid-air.  
  When in mid-air, the player can jump additional times.  
  Extra jumps are recharged when standing or sliding on a solid wall.
- __Wall Jump__ _Non-stackable_  
  Grants the ability to jump off walls.  
  When sliding on a wall, press the jump button to jump off of the wall.
- __Bullet Deflect__ _Stackable_  
  Grants the ability to deflect enemy bullets by attacking them.  
  Deflected bullets deal one unit of damage.  
  Each pickup increases the deflected bullet's damage by one unit.
- __Damage Up__ _Stackable_  
  Increases attacking damage by one unit.
- __Speed Up__ _Stackable_  
  Increases the player's speed.
- __Jump Height Up__ _Stackable_  
  Increases the maximum height of the player's jump.
- __Bullet Shoot__ _Stackable_  
  Grants the ability to shoot bullets when attacking.  
  Bullets deal one unit of damage.  
  Each pickup increases the bullet's damage by one unit.
- __Dash__ _Stackable_  
  Each pickup grants the player to use a dash in mid-air.  
  Dashes can only be done in mid-air.  
  Dashes are recharged when standing or sliding on a solid wall.  
  See the [Controls](#controls)
- __Knockback Up__ _Stackable_  
  When attacking an enemy, knockback throws the enemy backwards.  
  Each pickup adds additional knockback strength.

## Changing the window resolution
Included with the game's binary, is a file called `resolution.txt`.  
You can specify a custom window resolution in this file.  
Simply write your resolution in the format ...
```
WIDTH x HEIGHT
```
... where `WIDTH` and `HEIGHT` are positive integer numbers.  
Any lines starting with `#` are seen as comments and are ignored.  
See the `resolution.txt` file itself for more information and some preset resolutions.

## Post-Jam Version
After the jam ended at the end of April, we continued working on the game  
with the goal to release a finished product (like always).  
This time, we are proud to say, that we _actually finished this game_! (*__TODO:__ Not quite yet...*).

### Post-Jam Version Downloads
Post-jam-version binaries are also available on Google Drive.  
You can pick the latest version for your platform from this [Google Drive directory][bin-dir].  
(*__TODO:__ List latest versions here...*)

---

## Jam Version
To see the jam version, visit out our [Ludum Dare page][ludumdare], or  
check out the [`LD44-release`][LD44-release] tag.  

As always with our jam games, we never properly finish.  
This jam's game was _especially unfinished_.  
If you intend to check out our game, we recommend trying our post-jam version.

### Jam Version Downloads
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

---

## Licensing
This project, including all of its code and assets (such as images, audio, level files),  
excluding fonts, are distributed under the terms of the [MIT License][MIT-license].  

### Fonts
The fonts (found under `resources/fonts`) remain unchanged,  
and are licensed individually by their authors.

#### Undefined Medium
- Located at `resources/fonts/undefined-medium.ttf`
- [OFL License][undefined-medium-license]
- [Website][undefined-medium-website]
- [GitHub Repository][undefined-medium-github]
#### Ruji's Handwriting
- Located at `resources/fonts/rujis-handwriting.ttf`
- [OFL License][OFL-license]
- [Author's Website][rujis-handwriting-website]


### SDL2
This project uses and redistributes an unchanged copy of [SDL2],  
for controller support with [amethyst]'s `sdl_controller` feature.  
`SDL2.dll` is distributed under the [zlib license][SDL2-license].

[ludumdare]:                 https://ldjam.com/events/ludum-dare/44/veggietartar
[LD44-release]:              https://github.com/Noah2610/LD44/tree/LD44-release
[bin-jam-windows]:           https://drive.google.com/file/d/1RtQ8vsJFH75WyZHTa1W_vxzchw6OhA7Y/view
[bin-jam-linux]:             https://drive.google.com/file/d/1xffSHQmDppZ4KGcbApzIFt4l_XvT2pX2/view
[bin-dir]:                   https://drive.google.com/open?id=1XNMvBsOJkbbii6jp9T7Du9mNnso00Axn
[Rust]:                      https://www.rust-lang.org/
[Amethyst]:                  https://amethyst.rs/
[Deathframe]:                https://github.com/Noah2610/deathframe
[Vim]:                       https://www.vim.org/
[GitHub]:                    https://github.com/Noah2610/LD44
[Gimp]:                      https://www.gimp.org/
[Aseprite]:                  https://www.aseprite.org/
[BoscaCeoil]:                https://boscaceoil.net/
[FLStudio]:                  https://www.image-line.com/flstudio
[Tiled]:                     https://www.mapeditor.org/
[GoogleDrive]:               https://drive.google.com/
[Trello]:                    https://trello.com/b/hkBWXYt9
[MIT-license]:               https://github.com/Noah2610/LD44/blob/master/LICENSE
[SDL2]:                      https://www.libsdl.org/
[SDL2-license]:              https://www.libsdl.org/license.php
[undefined-medium-website]:  https://undefined-medium.com/
[undefined-medium-github]:   https://github.com/andirueckel/undefined-medium
[undefined-medium-license]:  https://github.com/andirueckel/undefined-medium/blob/master/OFL.txt
[OFL-license]:               https://scripts.sil.org/OFL
[rujis-handwriting-website]: http://rujic.net/

[items-spritesheet]:         ./resources/spritesheets/items.png
