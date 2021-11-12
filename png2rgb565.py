#!/usr/bin/python

# MIT License

# Copyright (c) 2019 jpfwong

# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:

# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.

# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

# based on https://github.com/jimmywong2003/PNG-to-RGB565 with slight modification
# usage python png2rgb565.py ./vulcan/assets/volcano.png ./vulcan/assets/volcano.raw

import sys
import os

from PIL import Image
from PIL import ImageDraw
import struct

isSWAP = False
# isSWAP = True

def main():

    len_argument = len(sys.argv)
    filesize = 0
    if (len_argument != 3):
      print ("")
      print ("Correct Usage:")
      print ("\tpython png2rgb565.py <png_file> <binary_file>")
      print ("")
      sys.exit(0)

    try:
        im=Image.open(sys.argv[1])
        print ("/* Image Width:%d Height:%d */" % (im.size[0], im.size[1]))
    except:
        print ("Fail to open png file ", sys.argv[1])
        sys.exit(0)

    image_height = im.size[1]
    image_width = im.size[0]

    try:
        binoutfile = open(sys.argv[2],"wb")
    except:
        print ("Can't write the binary file %s" % sys.argv[2])
        sys.exit(0)

    pix = im.load()  #load pixel array
    for h in range(image_height):
        for w in range(image_width):
            if w < im.size[0]:
                R=pix[w,h][0]>>3
                G=pix[w,h][1]>>2
                B=pix[w,h][2]>>3

                rgb = (R<<11) | (G<<5) | B

                binoutfile.write(struct.pack('H', rgb))
            else:
                rgb = 0

    binoutfile.close()

    print ("PNG file \"%s\"" % sys.argv[1], "converted to \"%s\"" % sys.argv[2])

if __name__=="__main__":
  main()