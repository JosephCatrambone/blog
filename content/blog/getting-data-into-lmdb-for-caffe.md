+++
title = "Getting Data into LMDB for Caffe"
date = "2015-02-05"

[taxonomies]
tags=["Programming"]
+++

This borders on blogspam, but I found it so useful that I can't help but share.Evan Shelhamer shared [here](https://groups.google.com/forum/#!msg/caffe-users/19XfmJqg34Q/0qBxNwEeSNkJ) the format that Caffe expects its input data to be.

```

import caffe
import lmdb

in_db = lmdb.open('image-lmdb', map_size=int(1e12))
with in_db.begin(write=True) as in_txn:
    for in_idx, in_ in enumerate(inputs):
        im = caffe.io.load_image(in_)
        im_dat = caffe.io.array_to_datum(im.transpose((2, 0, 1)))
        in_txn.put('{:0>10d}'.format(in_idx), im_dat.SerializeToString())
in_db.close()
</code>
```
