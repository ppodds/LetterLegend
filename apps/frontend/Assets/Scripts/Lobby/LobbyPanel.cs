using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using IO.Net;
using Protos.Lobby;

public class LobbyPanel : MonoBehaviour
{
    public GameObject startPanel;
    public GameObject lobbyPanel;
    public GameObject roomPanel;
    public GameObject lobbyItem;
    public Transform lobbyListTransform;

    private async void Join(uint maxPlayers)
    {
        var lobby = await GameManager.Instance.GameTcpClient.CreateLobby(maxPlayers);
        // TODO need to check lobby create success?
        if (lobby != null) EnterRoom(lobby);
    }

    public void CreateLobby()
    {
        uint maxPlayers = 4;
        Join(maxPlayers);
    }

    public void EnterRoom(Lobby lobby)
    {
        roomPanel.SetActive(true);
        var t = roomPanel.GetComponent<RoomPanel>();
        t.Lobby = lobby;
        t.UpdateRoom();
        gameObject.SetActive(false);
    }

    private async void OnEnable()
    {
        var lobbyList = await GameManager.Instance.GameTcpClient.GetLobbies();
        if (lobbyList == null)
        {
            return;
        }

        foreach (var lobbyInfo in lobbyList)
        {
            var t = Instantiate(lobbyItem, lobbyListTransform).GetComponent<LobbyItem>();
            t.LobbyInfo = lobbyInfo;
            t.UpdateText();
        }
    }

    private void OnDisable()
    {
        for (var i = 0; i < lobbyListTransform.childCount; i++) Destroy(lobbyListTransform.GetChild(i).gameObject);
    }

    public async void BackToStart()
    {
        await GameManager.Instance.GameTcpClient.Disconnect();
        GameManager.Instance.startPanel.gameObject.SetActive(true);
        gameObject.SetActive(false);
    }

    private void Awake()
    {
        GameManager.Instance.SetLobbyPanel(this);
    }
}