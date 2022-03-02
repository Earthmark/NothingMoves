# nothing_moves
A game jam entry for the bevy jam.

The hypotheses of this game is to fold higher dimensional structures into lower dimensional structures using matrix rotations. The lower dimensional structures are mapped into bevy as models, and rendered.

This is going to be using an old idea and migrating it into Bevy, later on I'll see if this can be migrated into objects having traversable multidimensional facets.


## Navigation Notes
On axis flip:
1. Spawn new axis as invisible with rotation target
2. Start rotating current target and shift to invisible.
3. On end of rotation remove old transform.

On move
1. Set destination of transform.
