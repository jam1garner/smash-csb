# csb_lib

A Rust library for reading and writing `commonsoundtable.csb` files from Super Smash Bros. Ultimate.

## csb_yaml

A command-line program for creating and editing `commonsoundtable.csb` files using YAML. Drag and drop a `commonsoundtable.csb` file onto the executable to create a YAML file. Drag and drop a properly structured YAML file onto the executable to create a `commonsoundtable.csb` file. YAML files are text files, so they can be viewed and edited in any text editor.

Sample output from a `commonsoundtable.csb` file:

```yaml
entries:
- fighter_kind: mario
  sound_table:
  - vc_mario_missfoot01
  - vc_mario_missfoot02
  - vc_mario_damage_twinkle
  - vc_mario_cheer
- fighter_kind: donkey
  sound_table:
  - vc_donkey_missfoot01
  - vc_donkey_missfoot02
  - vc_donkey_damage_twinkle
  - vc_donkey_cheer
```

### Usage

The latest prebuilt binary for Windows is available in [Releases](https://github.com/jam1garner/smash-csb/releases/latest).

Download the latest set of [labels](https://github.com/ultimate-research/param-labels/blob/master/commonsoundtable/Labels.txt) and have them placed beside the executable when dragging and dropping or include them in the command when converting to YAML. Missing labels will result in all sound labels appearing as hashes.

`csb_yaml <input> [output]`<br>
`csb_yaml <input> [output] [label]`<br>
`csb_yaml commonsoundtable.csb commonsoundtable.yaml`<br>
`csb_yaml commonsoundtable.csb commonsoundtable.yaml -l Labels.txt`<br>
`csb_yaml commonsoundtable.yaml commonsoundtable.csb`<br>
