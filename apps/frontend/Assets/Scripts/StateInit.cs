using System;
using System.Collections;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using IO.Net;
using Protos.Game;
using Protos.Lobby;
using UnityEngine;

public class StateInit : State
{
    private readonly Board _board;

    public override void ExecAsync(byte[] buf)
    {
        Client.TransitionTo(new StateLobbyBroadcast(Client));
    }

    public StateInit(GameTcpClient gameTcpClient) : base(gameTcpClient)
    {
    }
}