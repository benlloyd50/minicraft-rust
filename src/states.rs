/*
Some talk about how I will implement states in the game and how many we need

    *Start* -> Asset Loading --> Main Menu

        do:
        --> New Game --> Game Loading -->
        or:
        --> Load Game --> Game Loading -->

    then either:
        Game Loop --> Save

        do:
        --> Main Menu
        or:
        --> Quit Application

    App States Needed:
        Asset Loading
        Main Menu
        New Game
        Load Game
        Game Load
        Game Loop
        Saving
*/

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    AssetLoad,
    MainMenu,
    NewGame,
    LoadGame,
    GameLoad,
    InGame,
    Save,
}
