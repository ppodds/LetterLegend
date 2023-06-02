using System;
using System.Collections;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using IO.Net;
using Protos.Game;
using Protos.Lobby;
using UnityEngine;

public class StateGameBroadcast : State
{
    private readonly Board _board;

    public override void ExecAsync(byte[] buf)
    {
        var res = GameBroadcast.Parser.ParseFrom(buf);
        // TODO: send message to board main thread
        // _board.SetGameState((int)res.Event, res.Lobby);
        if (res.Event == GameEvent.Leave)
            Client.TransitionTo(new StateLobbyBroadcast(Client));
    }

    public StateGameBroadcast(GameTcpClient gameTcpClient) : base(gameTcpClient)
    {
        _board = GameObject.Find("Board").GetComponent<Board>();
    }
}