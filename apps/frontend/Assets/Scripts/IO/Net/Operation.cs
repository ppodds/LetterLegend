namespace IO.Net
{
    public enum Operation
    {
        Connect,
        Disconnect,
        Heartbeat,
        CreateLobby,
        JoinLobby,
        QuitLobby,
        ListLobby,
        Ready,
        StartGame,
        SetTile,
        FinishTurn,
        GetNewCard,
        Cancel,
        Exit
    }
}