using System;
using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;
using UnityEngine.Events;
using UnityEngine.UI;

public class StartPanel : MonoBehaviour
{
    public GameObject startPanel;
    public GameObject lobbyPanel;
    public GameObject roomPanel;
    public TMP_InputField  inputField;
    public Button connectButton;
    private string _playerName;
    
    private void Awake()
    {
        SwitchToStart();
        SwitchToLobby();
    }
    
    public void SwitchToLobby()
    {
        startPanel.SetActive(false);
        lobbyPanel.SetActive(false);
        roomPanel.SetActive(true);
    }
    
    private void SwitchToStart()
    {
        startPanel.SetActive(true);
        lobbyPanel.SetActive(false);
        roomPanel.SetActive(false);
    }
    
    public void SetUserName(string s)
    {
        _playerName = s;
        Debug.Log(_playerName);
    }
    
    void Update()
    {
        
    }
}
