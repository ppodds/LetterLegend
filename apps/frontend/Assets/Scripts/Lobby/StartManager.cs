using System;
using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;
using UnityEngine.Events;
using UnityEngine.UI;

public class StartManager : MonoBehaviour
{
    public GameObject startPanel;
    public GameObject lobbyPanel;
    public GameObject roomPanel;
    public TMP_InputField  inputField;
    public Button connectButton;

    private string _playerName;
    // Start is called before the first frame update
    private void Awake()
    {
        SwitchToStart();
        inputField.onEndEdit.AddListener(SetUserName);
        connectButton.onClick.AddListener(SwitchToLobby);
    }
    private void SwitchToStart()
    {
        startPanel.SetActive(true);
        lobbyPanel.SetActive(false);
        roomPanel.SetActive(false);
    }
    private void SwitchToLobby()
    {
        startPanel.SetActive(false);
        lobbyPanel.SetActive(true);
        roomPanel.SetActive(false);
    }
    private void SetUserName(string s)
    {
        _playerName = s;
        Debug.Log(_playerName);
    }
    
    // private void SwitchToLobby()
    // {
    //     lobbyManager.switch()
    // }
    // public void 
    // Update is called once per frame
    void Update()
    {
        
    }
}
