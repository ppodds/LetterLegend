using System;
using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;
using UnityEngine.UI;

public class LobbyItem : MonoBehaviour
{
    private TMP_Text lead;
    private TMP_Text people;
    
    // public void UpdateText()
    // {
    //     lead.SetText("Lead: " + Lobby.Lead.Id);
    //     people.SetText(Lobby.CurPeople + " / " + Lobby.MaxPeople);
    // }
    //
    // private async Task JoinRoomTask()
    // {
    //     var lobby = await GameManager.Instance.GameTcpClient.JoinLobby(Lobby);
    //     if (lobby == null)
    //     {
    //         GameManager.Instance.toast.PushToast("Join failed");
    //         return;
    //     }
    //
    //     if (await GameManager.Instance.ConnectToLobby(lobby))
    //         GameManager.Instance.lobbyPanel.ShowPrepareRoom(lobby);
    // }
    //
    // public void JoinRoom()
    // {
    //     JoinRoomTask();
    // }
}
