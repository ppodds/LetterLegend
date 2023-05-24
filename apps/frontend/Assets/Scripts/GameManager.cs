using System;
using IO.Net;
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
    public uint PlayerID { get; private set; }
    public GameTcpClient GameTcpClient { get; private set; }

    public static GameManager Instance { get; private set; }

    public Server Server { get; set; }

    private void Awake()
    {
        startPanel.gameObject.SetActive(true);
        lobbyPanel.gameObject.SetActive(false);
        roomPanel.gameObject.SetActive(false);
        if (Instance != null)
        {
            Destroy(gameObject);
            return;
        }

        Instance = this;
        DontDestroyOnLoad(gameObject);
    }

    public void SwitchScene(string scene)
    {
        startPanel.gameObject.SetActive(scene=="startPanel");
        lobbyPanel.gameObject.SetActive(scene=="lobbyPanel");
        roomPanel.gameObject.SetActive(scene=="roomPanel");
    }
    
    public bool ConnectToServer()
    {
        GameTcpClient = new GameTcpClient(Server.Host, Server.TcpPort);
        //TODO: check success
        return true;
    }

    public async void StartGame()
    {
        var res = await GameTcpClient.Start();
        SceneManager.LoadScene("InGame");
    }
}