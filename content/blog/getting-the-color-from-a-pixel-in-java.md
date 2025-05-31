+++
title = "Getting the Color from a Pixel in Java"
date = "2014-06-12"

[taxonomies]
tags=["Programming"]
+++

Images are often stored with their pixels formatted as RGBA values, with each channel taking 8 bits. Together, these 4x eight-bit numbers make up a 32-bit integer. The color dark red, for example, would be 0x990000FF. The 0x99 denotes a reddish color (as it is in the red channel) and the 0xFF indicates that the color is not transparent. 0x99000000 would not be visible, as its opacity is 0x00. Every time I find myself working with images in Java, the problem of always-signed numbers arises. 0xFF000000, when the top-most channel is extracted, sometimes will return -1, as 0xFF can be interpreted in a few ways if you're not explicit. This is the code I've been using to extract colors from channels.

```

/*** getChannel
 * Returns a byte from the pixel by channel.
 * @param pixel
 * @param channel
 * @return
 */
public static int getChannel(int pixel, char channel) {
	if(channel == 'r' || channel == 'R') {
		return byteToUInt((byte)((pixel & 0xFF000000) >>> 24));
	} else if(channel == 'g' || channel == 'G') {
		return byteToUInt((byte)((pixel & 0x00FF0000) >>> 16));
	} else if(channel == 'b' || channel == 'B') {
		return byteToUInt((byte)((pixel & 0x0000FF00) >>> 8));
	} else if(channel == 'a' || channel == 'A') {
		return byteToUInt((byte)(pixel & 0x000000FF));
	} else {
		System.err.println("Unrecognized channel: " + channel);
		return 0;
	}
}
</code>
```
