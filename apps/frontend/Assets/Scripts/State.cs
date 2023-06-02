using System.Collections;
using System.Collections.Generic;
using System.Threading.Tasks;
using IO.Net;
using Protos.Lobby;
using UnityEngine;

public abstract class State
{
    protected GameTcpClient Client { get; }

    protected State(GameTcpClient gameTcpClient)
    {
        Client = gameTcpClient;
    }

    public abstract void ExecAsync(byte[] buf);
}