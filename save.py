# benchmarking TODO
# layering TODO
# async operations future TODO async or concurrent 
# cache blocks: layers: synch inode: write thru cache, wrrite behiond cache(synch)

# what does synch do?
# sizse of cache? diminishing returns

# lrucache? clock strategy is approx for speed...
# cc fs: io concurrency: rw lock on inodes 
# layers...

# impl caching layer 
# both write thru and write behind
# synch will flush dirty blocks in write behind 
# synch(specific inode), sync(-1) flush all dirty blocks
# fsync 
# clock algorithm for cache evicton
