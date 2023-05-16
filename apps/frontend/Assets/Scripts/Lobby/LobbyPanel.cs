using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using IO.Net;
public class LobbyPanel : MonoBehaviour
{
    public GameObject startPanel;
    public GameObject lobbyPanel;
    public GameObject roomPanel;
    public GameObject lobbyItem;
    public Transform lobbyListTransform;
    public GameTcpClient tcpClient { get; set; }
    public void SwitchToStart()
    {
        startPanel.SetActive(true);
        lobbyPanel.SetActive(false);
        roomPanel.SetActive(false);
    }
    
    public void SwitchToRoom()
    {
        startPanel.SetActive(false);
        lobbyPanel.SetActive(false);
        roomPanel.SetActive(true);
    }
    
    private void Awake()
    {
        // for (int i = 0; i < 5; i++)
        // {
        //     var t = Instantiate(lobbyItem, lobbyListTransform).GetComponent<LobbyItem>();
        //     t.GetComponent<Button>().onClick.AddListener(SwitchToRoom);
        // }
    }

    private async void OnEnable()
    {
        var lobbyList = await tcpClient.GetLobby();
        foreach (var lobbyInfo in lobbyList)
        {
            var t = Instantiate(lobbyItem, lobbyListTransform).GetComponent<LobbyItem>();
            t.LobbyInfo = lobbyInfo;
            t.UpdateText();
        }
        // t.GetComponent<Button>().onClick.AddListener(SwitchToRoom);
    }
    
    private void OnDisable()
    {
        for (var i = 0; i < lobbyListTransform.childCount; i++) Destroy(lobbyListTransform.GetChild(i).gameObject);
    }
}
