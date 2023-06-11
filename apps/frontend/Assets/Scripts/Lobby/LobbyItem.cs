using System;
using System.Collections;
using System.Collections.Generic;
using System.Threading.Tasks;
using Protos.Lobby;
using TMPro;
using UnityEngine;
using UnityEngine.UI;

public class LobbyItem : MonoBehaviour
{
    public TMP_Text lead;
    public TMP_Text people;
    public LobbyInfo LobbyInfo { get; set; }

    public void UpdateText()
    {
        lead.SetText("Lead: " + LobbyInfo.Id);
        people.SetText(LobbyInfo.CurrentPlayers + " / " + LobbyInfo.MaxPlayers);
    }

    private async Task JoinRoomTask()
    {
        var lobby = await GameManager.Instance.GameTcpClient.JoinLobby(LobbyInfo.Id);
        if (lobby == null)
        {
            Debug.Log("Join failed");
            return;
        }
        
        GameManager.Instance.lobbyPanel.EnterRoom(lobby);
    }

    public async void JoinRoom()
    {
        await JoinRoomTask();
    }
}