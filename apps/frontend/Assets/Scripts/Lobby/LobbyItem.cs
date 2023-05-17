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
    private TMP_Text _lead;
    private TMP_Text _people;
    public LobbyInfo LobbyInfo { get; set; }
    
    public void UpdateText()
    {
        _lead.SetText("Lead: " + LobbyInfo.Id);
        _people.SetText(LobbyInfo.CurrentPlayers + " / " + LobbyInfo.MaxPlayers);
    }
    
    private async Task JoinRoomTask()
    {
        var lobby = await GameManager.Instance.GameTcpClient.JoinLobby(LobbyInfo.Id);
        if (lobby == null)
        {
            // GameManager.Instance.toast.PushToast("Join failed");
            Debug.Log("Join failed");
            return;
        }
        GameManager.Instance.lobbyPanel.EnterRoom(lobby);
    }
    
    public void JoinRoom()
    {
        JoinRoomTask();
    }
}
