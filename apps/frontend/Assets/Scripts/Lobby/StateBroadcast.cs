using System;
using System.Threading;
using System.Threading.Tasks;
using IO.Net;
using Protos.Lobby;
using UnityEngine;

public class StateBroadcast : State
{
    private readonly RoomPanel _roomPanel;
    public StateBroadcast(GameTcpClient gameTcpClient) : base(gameTcpClient)
    {
        _roomPanel = GameObject.Find("RoomPanel").GetComponent<RoomPanel>();
    }
    public override async Task ExecAsync(byte[] buf)
    {
        var res = LobbyBroadcast.Parser.ParseFrom(buf);
        switch (res.Event)
        {
            case LobbyEvent.Join:
                _roomPanel.ClearList();
                _roomPanel.Lobby = res.Lobby;
                _roomPanel.UpdateRoom();
                break;
            case LobbyEvent.Leave:
                _roomPanel.ClearList();
                _roomPanel.Lobby = res.Lobby;
                _roomPanel.UpdateRoom();
                break;
            case LobbyEvent.Destroy:
                _roomPanel.lobbyPanel.SetActive(true);
                _roomPanel.gameObject.SetActive(false);
                break;
            case LobbyEvent.Start:
                GameManager.Instance.StartGame();
                //TODO switch to InGame State
                break;
            default:
                throw new ArgumentOutOfRangeException();
        }
    }

    public void SwitchToResponse()
    {
        Client.TransitionTo(new StateResponse(Client));
    }
}