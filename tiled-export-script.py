"""
hello-amethyst-platformer tiled export script
"""

import json
import re
from tiled import *

class Tile:
    def __init__(self, tile_map, layer, x, y, cell):
        self.cell = cell
        self.layer = layer
        ts = self.tileset()
        # ORIGIN at TOP-LEFT
        # self.pos = { "x": x * ts.tileWidth(), "y": y * ts.tileHeight() }
        # ORIGIN at BOTTOM-LEFT
        self.pos = { "x": x * ts.tileWidth(), "y": (tile_map.height() * tile_map.tileHeight()) - (y * ts.tileHeight()) }

    def tile(self):
        return self.cell.tile()

    def tileset(self):
        return self.cell.tileset()

    def data(self):
        data = {}
        data["id"] = self.tile().id()
        data["pos"] = self.pos
        data["ts"] = ".".join(self.tileset().imageSourceString().split("/")[-1].split(".")[0 : -1])
        data["properties"] = { **properties_of(self.layer), **properties_of(self.tile()) } # Merge layer and tile properties, tile properties getting precedence
        return data

    def display(self):
        out  = "\nTILE "
        out += str(self.pos["x"]) + "," + str(self.pos["y"])
        out += "\nid: "               + str(self.tile().id())
        out += "\nw,h: "              + str(self.tile().width()) + "," + str(self.tile().height())
        out += "\nsolid: "            + str(self.tile().propertyAsString("solid"))
        out += "\ntileset name: "     + str(self.tileset().name())
        out += "\ntileset filename: " + str(self.tileset().fileName())
        out += "\n---"
        return out

class Tileset:
    def __init__(self, tileset):
        self.tileset = tileset

    def name(self):
        return self.tileset.name()

    def filename(self):
        return self.tileset.imageSourceString().split("/")[-1]

    def data(self):
        grid_size = self.tileset.gridSize()
        data = {}
        #data["name"] = self.tileset.name()
        data["image_filename"] = self.filename()
        data["tile_size"] = { "w": grid_size.width(), "h": grid_size.height() }
        data["properties"] = properties_of(self.tileset)
        return data

    def ron_data(self):
        tileset = self.tileset
        tile_size = { 'w': tileset.tileWidth(), 'h': tileset.tileHeight() }
        spritesheet_size = { 'w': tileset.imageWidth(), 'h': tileset.imageHeight() }
        spritesheet_size['w'] = spritesheet_size['w'] - spritesheet_size['w'] % tile_size['w']
        spritesheet_size['h'] = spritesheet_size['h'] - spritesheet_size['h'] % tile_size['h']
        content = '('
        # content += '\n  filename: "' + self.filename() + '",'
        content += '\n  spritesheet_width:  ' + str(spritesheet_size['w']) + ','
        content += '\n  spritesheet_height: ' + str(spritesheet_size['h']) + ','
        content += '\n  sprites: ['
        tiles_per_row = spritesheet_size['w'] / tile_size['w']
        tiles_per_col = spritesheet_size['h'] / tile_size['h']
        tiles_count = tiles_per_row * tiles_per_col
        for i in range(int(tiles_count)):
            row = int(i / tiles_per_row)
            col = int(i - row * tiles_per_row)

            content += '\n    ('
            content += '\n      x:      ' + str(tile_size['w'] * col) + ','
            content += '\n      y:      ' + str(tile_size['h'] * row) + ','
            content += '\n      width:  ' + str(tile_size['w']) + ','
            content += '\n      height: ' + str(tile_size['h']) + ','
            content += '\n    ),'
        content += '\n  ],'
        content += '\n)'
        return content

    def display(self):
        out  = "\nTILESET "
        out += str(self.tileset.name())
        out += "\nimage: "          + str(self.tileset.imageSourceString())
        out += "\nfilename: "       + str(self.tileset.fileName())
        out += "\ntile_size w, h: " + str(self.tileset.gridSize().width()) + ", " + str(self.tileset.gridSize().height())
        #out += "\nmethods: " + str(methods_of(self.tileset))
        return out

class Object:
    def __init__(self, tile_map, layer, obj):
        self.layer = layer
        # ORIGIN at TOP-LEFT
        # self.pos  = { "x": obj.x(), "y": obj.y() }
        # ORIGIN at BOTTOM-LEFT
        self.pos = { "x": obj.x(), "y": (tile_map.height() * tile_map.tileHeight()) - obj.y() }

        self.size = { "w": obj.width(), "h": obj.height() }
        self.name = obj.name()
        self.type = obj.type()
        self.obj  = obj

    def is_visible(self):
        return self.obj.isVisible()

    def data(self):
        data = {}
        data["name"] = self.name
        data["type"] = self.type
        data["pos"]  = self.pos
        data["size"] = self.size
        data["properties"] = { **properties_of(self.layer), **properties_of(self.obj) } # Merge layer and object properties, object properties getting precedence
        return data


    def display(self):
        out  = "\nOBJECT"
        out += "\nname: "    + str(self.obj.name())
        out += "\ntype: "    + str(self.obj.type())
        #out += "\nmethods: " + str(methods_of(self.obj))
        return out

class Export(Plugin):
    @classmethod
    def nameFilter(self):
        return "hello-amethyst-platformer export script (*.json)"

    @classmethod
    def shortName(self):
        return "hello-amethyst-platformer"

    @classmethod
    def write(self, tile_map, filepath_map):
        filepath_tileset = filepath_map.replace(".json", ".ts.json")
        filepath_base_spritesheet = "/".join(filepath_map.split("/")[0 : -1])
        tiles    = []
        tilesets = []
        objects  = []

        for tileset_idx in range(tile_map.tilesetCount()):
            data = tile_map.tilesetAt(tileset_idx).data()
            tilesets.append(Tileset(data))

        for layer_idx in range(tile_map.layerCount()):
            if isTileLayerAt(tile_map, layer_idx):
                layer = tileLayerAt(tile_map, layer_idx)
                if should_export_layer(layer):
                    for row in range(layer.height()):
                        for col in range(layer.width()):
                            cell = layer.cellAt(col, row)
                            if not cell.isEmpty():
                                tiles.append(Tile(tile_map, layer, col, row, cell))
            elif isObjectGroupAt(tile_map, layer_idx):
                layer = objectGroupAt(tile_map, layer_idx)
                if should_export_layer(layer):
                    for object_idx in range(layer.objectCount()):
                        tiled_obj = layer.objectAt(object_idx)
                        objects.append(Object(tile_map, layer, tiled_obj))

        tile_size = { "w": tile_map.tileWidth(), "h": tile_map.tileHeight() }
        level_size = { "w": tile_map.width() * tile_size["w"], "h": tile_map.height() * tile_size["h"] }

        json_data = { "map": { "level": { "size": level_size }, "tiles": [], "objects": [] }, "tilesets": {} }

        for tileset in tilesets:
            json_data["tilesets"][tileset.name()] = tileset.data()

        for tile in tiles:
            json_data["map"]["tiles"].append(tile.data())

        for obj in objects:
            if obj.is_visible():
                json_data["map"]["objects"].append(obj.data())

        with open(filepath_map, "w") as file_handle:
            print(json.dumps(json_data["map"]), file=file_handle)

        with open(filepath_tileset, "w") as file_handle:
            print(json.dumps(json_data["tilesets"]), file=file_handle)

        for tileset in tilesets:
            filepath = filepath_base_spritesheet + "/" + ".".join(tileset.filename().split(".")[0 : -1]) + ".ron"
            content = tileset.ron_data()
            with open(filepath, "w") as file_handle:
                print(content, file=file_handle)

        return True

# SPECIAL META PROPERTY: 'disabled', don't export this layer!
def should_export_layer(layer):
    properties = properties_of(layer)
    if "disabled" in properties and properties["disabled"]:
        return False
    return True

def properties_of(obj):
    properties = {}
    regex = re.compile(r"\s*")
    for key in obj.properties().keys():
        if key == "components":
            properties[key] = []
            vals = regex.sub("", obj.propertyAsString(key)).split(";")
            for val in vals:
                if val:
                    properties[key].append(val)
        else:
            val = obj.propertyAsString(key)
            if val == "true":
                val = True
            elif val == "false":
                val = False
            elif is_int(val):
                val = int(val)
            elif is_float(val):
                val = float(val)
            properties[key] = val
    return properties

def is_int(value):
    try:
        int(value)
        return True
    except ValueError:
        return False

def is_float(value):
    try:
        float(value)
        return True
    except ValueError:
        return False


def methods_of(obj):
    return [method_name for method_name in dir(obj)
            if callable(getattr(obj, method_name))]
    return None
