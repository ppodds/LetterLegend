using System;
using System.Threading;
using System.Threading.Tasks;
using IO.Net;
using Protos.Lobby;
using UnityEngine;

public class StateInit : State
{
    public StateInit(GameTcpClient gameTcpClient) : base(gameTcpClient)
    {
        
    }
    public override async Task ExecAsync(byte[] buf)
    {
        Client.TransitionTo(new StateBroadcast(Client));
    }
}