# Report
## Tested Environment
- Compiler: `rustc` 1.49.0-nightly (1773f60ea 2020-11-08)
- Operating System: Windows 10 Pro 20H2

## Works
- Implemented ray-triangle intersection based on the parallelogram intersection
- Implemented scene file loading function
- Implemented off file loading function
- Implemented AABB tree generation based on a given set of trangles using bottom up method
- Implemented ray-bounding box intersection function
- Implemented bounding box generation for triangles
- Implemented bounding box merging function
- Implemented ray-mesh intersection based on BVH acceleration

## Explaination
The ray-triangle intersection algorithm is taught in class, and it's quite similar with the one used in assignment 2 which renders the parallelogram.

Firstly I created the `Triangle` struct, which contains three `Vec3` points. `Triangle` has a function to calculate its centroid.

Since the math library I use doesn't provide convenient bounding box calculation, I wrote all the bounding box type on my own. I wrote functions like `expand(&mut self, tri: &Triangle)`, `intersect(&self, ray: &Ray)`, `centroid(&self)` and `merge(&self, box: &BBox)`.

The ray-bbox intersection function I took reference from this [article](https://tavianator.com/2015/ray_box_nan.html), which introduced an efficient way to calculate it.

The `load_off(filename: &str)` function is pretty straightforward. As some of the data we don't really need to use, we would just skip those lines. Firstly we skip first line, and we read the first number as `nv` from the second line. Then we read until line `1 + nv` and store these values as points. From then on, read the rest and store them as `Triangle`s. The `load_off` function returns a list of triangles.

Loading the scene is much simpler. We use a crate called `serde`, which provides some serialization/deserialization functions for the primitive types. We construct the `Scene` type according to a style which could be interpreted by `serde`, and then we don't have to write any more code.

The AABB tree generation part, we use the bottom-up method. Firstly we put every triangles into a list. In each iteration, we take the first one from the list, traverse the whole list to find which one is the nearest. The distance function is just the length between their centroids. Then we take the nearest point out of the list as well, and merge these two points into a new node, calculate its bounding box and insert it to the end of the list. Repeat this process until there is only one node in the list. That one is the generated root node.

About the ray-mesh intersection function, we have a traversal function. For each traverse, we firstly detect if it contains only one triangle. If yes, then calculate the ray-triangle intersection; otherwise calculate if the ray intersect with its bounding box. If intersected, then check its left child and right child respectively. If only one of them returns value, then that is the intersection point; if both of them return values, then we need to find which one is closer to the ray origin.

I don't have enough time to render the dragon, since it takes too much time to build the AABB tree for the bottom-up method.

To change the loading file, you could simply change the corresponding string in `main.rs`.
