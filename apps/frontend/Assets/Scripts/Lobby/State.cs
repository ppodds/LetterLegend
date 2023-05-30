using System.Collections;
using System.Collections.Generic;
using System.Threading.Tasks;
using IO.Net;
using Protos.Lobby;
using UnityEngine;

public abstract class State
{
    public GameTcpClient Client { get;}

    public State(GameTcpClient gameTcpClient)
    {
        Client = gameTcpClient;
    }
    // public abstract Task Handle();
    public abstract Task ExecAsync(byte[] buf);
}
