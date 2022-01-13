# PC Task

## The problem:
    There's a need to store data in a three dimensional array, sized roughly 60x60x60. All elements are f32. While populating the array, it is of paramount importance to keep the axes as they are, since otherwise the data would be i) tedious to generate and b) the code would be extremely hard to read. Hence, once populated, we have an array where the order of the axes is x, y, z. Now, the next step in the process requires the data to be represented in a three dimensional array with axes ordered x, z, y and consequently y and z need to be swapped.

1. What data structure(s) would you use?
2. How would you swap the two axes as fast as possible?
   
   In reality the data can naturally be just one continuous chunk of bytes, but the order of the axes needs to be reflected in the physical representation.

## Solution
- Use a 1-d array and use strides to manuever in nd space.
- Use strides and dimensions to swap axes and do transposes.

### Pros
- Time complexity O(n)



#### Resources used to educate myself
The first resource that I used to know more about how to represent a 3d array as a single continuous array.

- https://www.ibm.com/docs/en/essl/6.3?topic=sequences-how-stride-is-used-three-dimensional.

This resource laid out how strides and dimensions can be used to index a linear array with 3d coordinates.

The other resources I used are a series of articles that outline how NumPy uses strides to index 3d arrays.

- https://ajcr.net/stride-guide-part-3/
- https://ajcr.net/stride-guide-part-2/
- https://ajcr.net/stride-guide-part-1/

By using strides transposing and swapping axes is fast as manipulating the strides.