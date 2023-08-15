import os

images = os.listdir("../doc/images");
images.sort();

tex = open("a.tex", "r").read();

all_seen = True;

for s in images:
    if s not in tex:
        print(f"image {s} not used in the document!");
        all_seen = False;

if all_seen:
    print("All images used!");

