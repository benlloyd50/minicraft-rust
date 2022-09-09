Ideas for what we do for an inventory system

## Requirements
1. We need an inventory system
2. That could support flexible item types
3. Not worried about "furniture" (deployables) or crafting

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