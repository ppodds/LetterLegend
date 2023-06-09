using System;
using System.Collections;
using System.Collections.Generic;
using IO.Net;
using Protos.Game;
using Protos.Lobby;
using Protos.Player;
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
    private Player _mainPlayer;
    private Player _currentPlayer;
    private Player _nextPlayer;
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
        _mainPlayer = new Player();
        _currentPlayer = new Player();
        _nextPlayer = new Player();
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
        SetHandCards(res.Item1);
        SetPlayers(res.Item2, res.Item3);
    }

    public void QuitGame()
    {
        SceneManager.LoadScene("Lobby");
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

    public void SetPlayers(Player player1, Player player2)
    {
        _currentPlayer = player1;
        _nextPlayer = player2;
    }

    public void SetMainPlayer(Player player)
    {
        _mainPlayer = player;
    }

    public Player GetMainPlayer()
    {
        return _mainPlayer;
    }

    public Player GetCurrentPlayer()
    {
        return _currentPlayer;
    }

    public Player GetNextPlayer()
    {
        return _nextPlayer;
    }

    public void SetRoomPanel(RoomPanel tmpPanel)
    {
        roomPanel = tmpPanel;
    }

    public void SetLobbyPanel(LobbyPanel tmpPanel)
    {
        lobbyPanel = tmpPanel;
        roomPanel.gameObject.SetActive(false);
        lobbyPanel.gameObject.SetActive(true);
        startPanel.gameObject.SetActive(false);
    }

    public void SetStartPanel(StartPanel tmpPanel)
    {
        startPanel = tmpPanel;
    }
}