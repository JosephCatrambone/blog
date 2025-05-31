---
title: 'Removing Blank and Hidden Layers from an SVG'
description: ""
published: 2013-10-10
redirect_from: 
            - https://www.josephcatrambone.com/?p=355
categories: "Programming"
hero: ../../../defaultHero.jpg
---
We discovered our former graphics designer was including all the hidden layers in his SVG icons. Each icon, that is, had a copy of every other icon stored inside invisibly. The practical upshot is our dashboard icons were in excess of 4MB to load. I wrote a simple script to iterate through and strip invisible layers. For anyone that might find utility:

```
def remove_blank_layers(image_name):
  tree = ET.ElementTree();
  tree.parse(image_name);
  root = tree.getroot();
  children_to_remove = list();
  for child in root.getchildren():
  if child.attrib.get('display') == 'none':
    children_to_remove.append(child);
  for child in children_to_remove:
    root.remove(child);
  tree.write(image_name);
```
