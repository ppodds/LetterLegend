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

    private void TestInput()
    {
        _playerName = "hello world";
        _host = "127.0.0.1";
        _port = 45678;
    }

    public void SetInput()
    {
        _playerName = nameField.text;
        _host = hostField.text;
        _port = int.Parse(tcpPortField.text);
        TestInput();
    }
    
    public async void Login()
    {
        SetInput();
        GameManager.Instance.Server = new Server{Host = _host, TcpPort = _port};
        var task = GameManager.Instance.ConnectToServer();
        if (task)
        {
            GameManager.Instance.GameTcpClient.TransitionTo(new StateConnect(GameManager.Instance.GameTcpClient));
            await GameManager.Instance.GameTcpClient.ConnectAsync(_playerName);
        }
        
        gameObject.SetActive(false);
        lobbyPanel.SetActive(true);
    }
    
}
