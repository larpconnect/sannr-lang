# Sannr Design

Sannr is a fundamentally experimental language in basically taking a ML-style language,
bringing in the lessons of languages like Erlang and Eiffel, and making it easy for humans
to read and AI systems to generate. 

It combines a few ideas:

1. **Linear Typing** by default. This means that values cannot be _discarded_. They _must_ be used **exactly** once. This mostly comes
   up with things tied into resources, such as database connections, or where an allocation has taken place. (GHC Haskell)
2. **Refinement Types**. Emphasizing the concepts of (probably) Liquid Types (Liquid Haskell).
3. **One Right Way To Do Things**. The number of ways to express the same thing are strictly limited. Avoiding semantic and syntactic drift.
   (Python)
4. **Machine Thinking First**. Recognize that generative AIs are essentially fancy matrix multipliers. Errors, outputs, etc are predominately
   in structured formats that are easily consumed.


## Papers

Things to look into later. 

* Kawamata et al. [Answer Refinement Modification: Refinement Type System for Algebraic Effects and Handlers](https://dl.acm.org/doi/10.1145/3633280). 2024.
* Rondon et al. [Liquid Types](https://patrickrondon.com/research/papers/liquid-types-pldi08.pdf). 2008.
* Mandelbaum et al. [An Effective Theory of Type Refinements](https://www.cs.cmu.edu/~rwh/papers/effref/icfp03.pdf). 2003.
* Pombrio et al. [Resugaring: Lifting Evaluation Sequences through Syntactic Sugar](https://cs.brown.edu/research/plt/dl/resugaring/v1/resugar.pdf). 2014.
* Swamy et al. [Lightweight Monadic Programming in ML](https://www.cs.umd.edu/~mwh/papers/monadic.pdf). 2011.
* Swamy et al. [Verifying higher-order programs with the dijkstra monad](https://dl.acm.org/doi/abs/10.1145/2499370.2491978). 2013.
