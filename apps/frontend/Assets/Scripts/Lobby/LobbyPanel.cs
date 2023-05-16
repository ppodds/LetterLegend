using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
public class LobbyPanel : MonoBehaviour
{
    public GameObject startPanel;
    public GameObject lobbyPanel;
    public GameObject roomPanel;
    public GameObject lobbyItem;
    public Transform lobbyListTransform;
    
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
        for (int i = 0; i < 5; i++)
        {
            var t = Instantiate(lobbyItem, lobbyListTransform).GetComponent<LobbyItem>();
            t.GetComponent<Button>().onClick.AddListener(SwitchToRoom);
        }
    }
}
