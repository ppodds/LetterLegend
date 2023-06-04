using System;
using System.Collections.Generic;
using IO.Net;
using Protos.Game;
using Protos.Lobby;
using UnityEditor;
using UnityEngine;
using UnityEngine.SceneManagement;

public struct Server
{
    public string Host;
    public int TcpPort;
}

public class GameManager : MonoBehaviour
{
    public LobbyPanel lobbyPanel;
    public RoomPanel roomPanel;
    public StartPanel startPanel;
    [SerializeField] private GameObject menuUI;
    [SerializeField] private GameObject gameUI;
    public Transform playersParent;
    private Lobby _lobby;
    private float _heartBeatTimeBase;
    private float _heartBeatTime;
    public uint PlayerID { get; private set; }
    private List<HandCard> _handCards;
    public GameTcpClient GameTcpClient { get; private set; }

    public static GameManager Instance { get; private set; }

    public Server Server { get; set; }

    private void Awake()
    {
        if (Instance != null)
        {
            Destroy(gameObject);
            return;
        }

        Instance = this;
        DontDestroyOnLoad(gameObject);
        _heartBeatTimeBase = _heartBeatTime = Time.time;
        startPanel.gameObject.SetActive(true);
        lobbyPanel.gameObject.SetActive(false);
        roomPanel.gameObject.SetActive(false);
    }

    private void Update()
    {
        _heartBeatTime = Time.time;
        if (_heartBeatTime - _heartBeatTimeBase >= 20)
        {
            HeartBeat();
            _heartBeatTimeBase = _heartBeatTime;
        }
    }

    public bool ConnectToServer()
    {
        GameTcpClient = new GameTcpClient(Server.Host, Server.TcpPort);
        //TODO: check success
        return true;
    }

    public async void StartGame()
    {
        var res = await GameTcpClient.StartGame();
        SceneManager.LoadScene("InGame");
        SetHandCards(res);
    }

    private async void HeartBeat()
    {
        if (GameTcpClient != null && GameTcpClient.IsConnected())
            await GameTcpClient.HeartBeat();
    }

    public void SetHandCards(List<HandCard> handCards)
    {
        _handCards = handCards;
    }

    public List<HandCard> GetHandCards()
    {
        return _handCards;
    }
}