using IO.Net;
using Protos.Lobby;
using UnityEngine;
using UnityEngine.WSA;

public struct Server
{
    public string Host;
    public int TcpPort;
    public int UdpPort;
}

public class GameManager : MonoBehaviour
{
    // public LobbyPanel lobbyPanel;
    // public RoomPanel roomPanel;
    // [SerializeField] private GameObject menuUI;
    // [SerializeField] private GameObject gameUI;
    // public Toast toast;
    //
    // public Transform playersParent;
    //
    // private Lobby _lobby;
    // public uint PlayerID { get; private set; }
    //
    // public GameTcpClient GameTcpClient { get; private set; }
    //
    // public static GameManager Instance { get; private set; }
    //
    // public Server Server { get; set; }
    //
    // private void Awake()
    // {
    //     if (Instance != null)
    //     {
    //         Destroy(gameObject);
    //         return;
    //     }
    //
    //     Instance = this;
    //     DontDestroyOnLoad(gameObject);
    // }
}