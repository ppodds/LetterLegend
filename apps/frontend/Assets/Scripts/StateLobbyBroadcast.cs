using System;
using System.Collections;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using IO.Net;
using Protos.Lobby;
using UnityEngine;

public class StateLobbyBroadcast : State
{
    private readonly RoomPanel _roomPanel;
    public override void ExecAsync(byte[] buf)
    {
        var res = LobbyBroadcast.Parser.ParseFrom(buf);
        _roomPanel.SetLobbyState((int)res.Event, res.Lobby);
        if(res.Event == LobbyEvent.Start)
            Client.TransitionTo(new StateGameBroadcast(Client));
    }

    public StateLobbyBroadcast(GameTcpClient gameTcpClient) : base(gameTcpClient)
    {
        _roomPanel = GameObject.Find("RoomPanel").GetComponent<RoomPanel>();
    }
}
