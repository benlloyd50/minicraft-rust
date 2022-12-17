# Collider System | Status: Idea
## Requirements
1. We need a collider system for gameplay
2. Types of bodies:
  a. Player <- dynamic, moves around alot, can bump into furniture and move it
  b. Player-pushable (furniture) <- only moved by player entity bumping into it
  c. Player-interactable <- idealy sends a message to the entity that hits it to do some logic, items trigger pickup, monsters trigger damage
  d. Static (world tiles) <- dont move, but might be removed at runtime
3. A way to detect collisions and deal with them, some sort of event reading
4. World Tiles need an optimized solution for how they are created.
  - The problem is creating a lot of colliders takes a long time
  - Mitigation is to create larger colliders that would mean less colliders need to be made
  - Something like stone though may be removed and the colliders should update
5. Player should be able to slide past the colliders and not get stuck

## Bulk Collider Creation
1. The colliders must be able to broken up after they are created
2. There is the plate approach where there is a collider for each layer of a blocking

# Inventory System | Status: Working Prototype
## Requirements
1. We need an inventory system
2. That could support flexible item types
3. Not worried about "furniture" (deployables) or crafting
4. Try to fit the aesthetic of minicraft

## Inventory
- Holds a defined amount of 'items'
- Add and Remove items from various ways
    1. picking up from ground
    2. taking out of a chest
    3. dropping out of inventory
    4. using in a crafting recipe
- Need a way to query the inventory and know what we have, item components!
- Stack items and fill inventory slots on adding them, stack size of ___

## Items
- Items are entities
- Item Component
- More components to give the item different effects
    1. Wiedable
    2. Damage
    3. Effective Against 
        - (types of components that will be on other things)
    4. Heals
    5. Light source
    6. Stackable (dne means it stacks to 1)
- Can clone the item to put in an inventory
- Exist as an unlimited amount, are seperated only when put into an inventory so it has no logic of stack size
- Item database to spawn items from when adding to drops of things
    - needs ids for items and ability to ask for by name
