initSidebarItems({"struct":[["HeapsPermutator","Heaps PermutatorThe permutator will not try to find an intelligent ordering. It implements the Heaps algorithm. Before the algorithm starts, the data is sorted by the given ordering."],["LexicographicPermutator","Ordered PermutatorThe `LexicographicPermutator` generates all permutations in lexicographic order. The ordering can be chosen using `ModifierOrdering`, which must be a `CompleteOrdering`."],["MultipleSwapPermutator","Multiple-Swap PermutatorThis is a permutator that is based on another permutator, calling the next method multiple times in order to have larger differences between each iteration. The number of times to call next, before we return the element, is determined by taking the next prime number, larger than the total number of elements to produce the permutator. This way, we are guaranteed that we return each element exactly once."],["RandomTreePermutator","Random Tree PermutatorThe random tree permutator is similar to the `TreePermutator`, but with the difference that it is shuffled every time a new branch of the tree is entered."],["SJTPermutator","Steinhaus-Johnson-Trotter PermutatorThis is an implementation of the Steinhaus-Johnson-Trotter Permutator Algorithm as described here. It is implemented using Even's speedup. Additionally, it is implemented for any type `T`, which implements `Copy`."],["TreePermutator","Tree PermutatorThe tree permutator is a permutator, which returns a sequence identical to the `LexicographicPermutator`. However, it does not require a `CompleteOrdering`, since it does not rely on the ordering of the elements to compute the next element. Instead, it traverses a tree, and stores which elements are remaining at each position. This means that you can pass in an arbitrary ordering. This ordering will determine the initial ordering of the array. Afterwards, it returns the sequence in a lexicographic order based on the index of the elements after they have been sorted."]],"trait":[["Permutator","Permutator trait"],["PermutatorItem","This is an empty trait to tell the compiler which types can be returned by the Permutator Iterator."]]});