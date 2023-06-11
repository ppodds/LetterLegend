using System;
using System.Collections;
using System.Collections.Generic;
using System.Threading.Tasks;
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
    public TMP_InputField hostField;
    public TMP_InputField tcpPortField;
    public TMP_InputField udpPortField;
    public TMP_InputField nameField;
    public Button connectButton;
    private string _playerName;
    private string _host;
    private int _port;

    public void SetInput()
    {
        _playerName = nameField.text;
        _host = hostField.text;
        _port = int.Parse(tcpPortField.text);
    }

    public async void Login()
    {
        SetInput();
        GameManager.Instance.Server = new Server { Host = _host, TcpPort = _port };
        var task = GameManager.Instance.ConnectToServer();
        if (task)
        {
            var res = await GameManager.Instance.GameTcpClient.ConnectAsync(_playerName);
            GameManager.Instance.SetMainPlayer(res);
        }

        gameObject.SetActive(false);
        lobbyPanel.SetActive(true);
    }

    private void Awake()
    {
        GameManager.Instance.SetStartPanel(this);
    }
}