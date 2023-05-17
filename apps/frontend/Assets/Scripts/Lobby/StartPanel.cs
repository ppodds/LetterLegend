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
        // SwitchToStart();
        // SwitchToLobby();
        // startPanel.gameObject.SetActive(true);
        // lobbyPanel.gameObject.SetActive(false);
        // roomPanel.gameObject.SetActive(false);
    }

    private void TestInput()
    {
        _playerName = "hello world";
        _host = "127.0.0.1";
        _port = 45678;
    }
    // public void SwitchToLobby()
    // {
    //     GameManager.Instance.SwitchScene("lobbyPanel");
    // }
    //
    // private void SwitchToStart()
    // {
    //     GameManager.Instance.SwitchScene("startPanel");
    // }
    
    public void SetInput()
    {
        _playerName = nameField.text;
        _host = hostField.text;
        _port = int.Parse(tcpPortField.text);
        TestInput();
        Debug.Log(_playerName);
        Debug.Log(_host);
        Debug.Log(_port);
    }
    
    public async void Login()
    {
        SetInput();
        GameManager.Instance.Server = new Server{Host = _host, TcpPort = _port};
        var task = GameManager.Instance.ConnectToServer();
        if (task)
        {
            await GameManager.Instance.GameTcpClient.ConnectAsync(_playerName);
        }
        gameObject.SetActive(false);
        lobbyPanel.SetActive(true);
    }
    
}
