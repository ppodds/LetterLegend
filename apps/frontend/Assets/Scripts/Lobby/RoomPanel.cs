using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using Protos.Lobby;
using UnityEngine;
using UnityEngine.SceneManagement;
using UnityEngine.UI;

public class RoomPanel : MonoBehaviour
{
    public GameObject startPanel;
    public GameObject lobbyPanel;
    public GameObject roomPanel;
    public GameObject playerItem;
    public Transform playerListTransform;
    public Button readyButton;
    public Lobby Lobby { get; set; }
    private int _state;
    private Queue<LobbyBroadcast> _lobbyBroadcasts;

    public void BroadcastEnqueue(LobbyBroadcast lobbyBroadcast)
    {
        lock (_lobbyBroadcasts)
        {
            _lobbyBroadcasts.Enqueue(lobbyBroadcast);
        }
    }

    public void Update()
    {
        LobbyBroadcast res;
        lock (_lobbyBroadcasts)
        {
            if (_lobbyBroadcasts.Count == 0)
            {
                return;
            }

            res = _lobbyBroadcasts.Dequeue();
        }

        switch (res.Event)
        {
            case LobbyEvent.Join:
                Lobby = res.Lobby;
                ClearList();
                UpdateRoom();
                break;
            case LobbyEvent.Leave:
                Lobby = res.Lobby;
                ClearList();
                UpdateRoom();
                break;
            case LobbyEvent.Destroy:
                lobbyPanel.SetActive(true);
                gameObject.SetActive(false);
                break;
            case LobbyEvent.Start:
                SceneManager.LoadScene("InGame");
                GameManager.Instance.SetHandCards(res.Cards.Cards_.ToList());
                GameManager.Instance.SetPlayers(res.CurrentPlayer, res.NextPlayer);
                break;
            default:
                throw new ArgumentOutOfRangeException();
        }
    }

    public async void BackToLobby()
    {
        await GameManager.Instance.GameTcpClient.QuitLobby();
        lobbyPanel.SetActive(true);
        gameObject.SetActive(false);
    }

    public void UpdateRoom()
    {
        foreach (var player in Lobby.Players)
        {
            var t = Instantiate(playerItem, playerListTransform).GetComponent<PlayerItem>();
            t.SetText(Lobby, player);
        }
    }

    private void OnEnable()
    {
        if (Lobby != null)
            UpdateRoom();
    }

    private void Awake()
    {
        GameManager.Instance.GameTcpClient.RoomPanel = this;
        _lobbyBroadcasts = new Queue<LobbyBroadcast>();
    }

    public async void SetReady()
    {
        await GameManager.Instance.GameTcpClient.SetReady();
        readyButton.GetComponent<Image>().color = Color.gray;
    }

    public void StartGame()
    {
        GameManager.Instance.StartGame();
    }

    private void ClearList()
    {
        for (var i = 0; i < playerListTransform.childCount; i++) Destroy(playerListTransform.GetChild(i).gameObject);
    }

    private void OnDisable()
    {
        ClearList();
    }
}