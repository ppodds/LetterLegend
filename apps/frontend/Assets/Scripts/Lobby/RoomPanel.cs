using System;
using System.Collections;
using System.Collections.Generic;
using Protos.Lobby;
using UnityEngine;
using UnityEngine.UI;
public class RoomPanel : MonoBehaviour
{
    public GameObject startPanel;
    public GameObject lobbyPanel;
    public GameObject roomPanel;
    public GameObject playerItem;
    public Transform playerListTransform;
    public Button readyButton;
    public Lobby Lobby { get; set; }

    public async void BackToLobby()
    {
        await GameManager.Instance.GameTcpClient.QuitLobby();
        lobbyPanel.SetActive(true);
        gameObject.SetActive(false);
    }
    
    public void UpdateRoom()
    {
        foreach (var player in Lobby.Players)
        {
            var t = Instantiate(playerItem, playerListTransform).GetComponent<PlayerItem>();
            t.SetText(Lobby, player);
        }
    }

    private void OnEnable()
    {
        if(Lobby!=null)
            UpdateRoom();
        
        // GameManager.Instance.GameTcpClient.Handle();
        // var inLobby = true;
        // while (inLobby)
        // {
        //     var res = await GameManager.Instance.GameTcpClient.WaitLobbyBroadcast();
        //     if(res == null) continue;
        //     switch (res.Event)
        //     {
        //         case LobbyEvent.Join:
        //             ClearList();
        //             Lobby = res.Lobby;
        //             UpdateRoom();
        //             break;
        //         case LobbyEvent.Leave:
        //             ClearList();
        //             Lobby = res.Lobby;
        //             UpdateRoom();
        //             break;
        //         case LobbyEvent.Destroy:
        //             lobbyPanel.SetActive(true);
        //             gameObject.SetActive(false);
        //             inLobby = false;
        //             break;
        //         case LobbyEvent.Start:
        //             GameManager.Instance.StartGame();
        //             inLobby = false;
        //             break;
        //         default:
        //             throw new ArgumentOutOfRangeException();
        //     }
        // }
    }

    public void SetReady()
    {
        ((StateBroadcast)GameManager.Instance.GameTcpClient.State).SwitchToResponse();
        // GameManager.Instance.GameTcpClient.Handle();
    }
    
    public void StartGame()
    {
        GameManager.Instance.StartGame();
    }

    public void ClearList()
    {
        for (var i = 0; i < playerListTransform.childCount; i++) Destroy(playerListTransform.GetChild(i).gameObject);
    }
    
    private void OnDisable()
    {
        ClearList();
    }
}
