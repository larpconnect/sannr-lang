# Sannr Design

Sannr is a fundamentally experimental language in basically taking a ML-style language,
bringing in the lessons of languages like Erlang and Eiffel, and making it easy for humans
to read and AI systems to generate. 

It combines a few ideas:

1. **Linear Typing** by default. This means that values cannot be _discarded_. They _must_ be used **exactly** once. This mostly comes
   up with things tied into resources, such as database connections, or where an allocation has taken place. (GHC Haskell)
2. **Design By Contract**. Functions are declared with the context on what you can expect built into them. This is then verified
   at _compile time_ (assuming I can get that to work, anyways). The goal here is not just to prove correctness, it is also to
   prevent agents from needing to read the entire function to know what it guarantees. (Eiffel, Ada)
3. **Refinement Types**. Essentially extending Design by Contract to types. Again, emphasizing a goal of minimizing the amount of context
   that has to be carried around. (F*, Ada)
4. **One Right Way To Do Things**. The number of ways to express the same thing are strictly limited. Avoiding semantic and syntactic drift.
   (Python)
5. **Machine Thinking First**. Recognize that generative AIs are essentially fancy matrix multipliers. Errors, outputs, etc are predominately
   in structured formats that are easily consumed.

