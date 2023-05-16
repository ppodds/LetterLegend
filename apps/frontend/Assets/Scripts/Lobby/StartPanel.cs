using System;
using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;
using UnityEngine.Events;
using UnityEngine.UI;
using IO.Net;
public class StartPanel : MonoBehaviour
{
    public GameObject startPanel;
    public GameObject lobbyPanel;
    public GameObject roomPanel;
    public TMP_InputField  hostField;
    public TMP_InputField  tcpPortField;
    public TMP_InputField  udpPortField;
    public TMP_InputField  nameField;
    public Button connectButton;
    private string _playerName;
    private string _host;
    private int _port;
    
    private void Awake()
    {
        SwitchToStart();
        // SwitchToLobby();
        TestInput();
    }

    private void TestInput()
    {
        _playerName = "hello world";
        _host = "127.0.0.1";
        _port = 45678;
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
    
    public void SetInput()
    {
        _playerName = nameField.text;
        _host = hostField.text;
        _port = int.Parse(tcpPortField.text);
        Debug.Log(_playerName);
        Debug.Log(_host);
        Debug.Log(_port);
    }
    
    public async void Login()
    {
        var tcpClient = new GameTcpClient(_host, _port);
        await tcpClient.ConnectAsync(_playerName);
        lobbyPanel.GetComponent<LobbyPanel>().tcpClient=tcpClient;
    }
    
}
