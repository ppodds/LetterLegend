using System;
using System.Threading;
using System.Threading.Tasks;
using IO.Net;
using Protos.Lobby;
using UnityEngine;

public class StateConnect : State
{
    private readonly RoomPanel _roomPanel;
    public StateConnect(GameTcpClient gameTcpClient) : base(gameTcpClient)
    {
        _roomPanel = GameObject.Find("RoomPanel").GetComponent<RoomPanel>();
    }
    public override async Task ExecAsync(byte[] buf)
    {
        var res = ReadyResponse.Parser.ParseFrom(buf);
        if (!res.Success)
        {
            throw new Exception("Set Ready failed");
        }
        Client.TransitionTo(new StateBroadcast(Client));
    }
}