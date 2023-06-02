using System;
using System.Collections;
using System.Collections.Generic;
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

    private enum State
    {
        Join = 0,
        Leave = 1,
        Destroy = 2,
        Start = 3,
        NoChange = 4
    }
    
    public void SetLobbyState(int state, Lobby lobby)
    {
        _state = state;
        Lobby = lobby;
    }

    public void Update()
    {
        var state = (State)_state;
        switch (state)
        {
            case State.Join:
                _state = (int)State.NoChange;
                ClearList();
                UpdateRoom();
                break;
            case State.Leave:
                _state = (int)State.NoChange;
                ClearList();
                UpdateRoom();
                break;
            case State.Destroy:
                _state = (int)State.NoChange;
                lobbyPanel.SetActive(true);
                gameObject.SetActive(false);
                break;
            case State.Start:
                // GameManager.Instance.StartGame();
                _state = (int)State.NoChange;
                SceneManager.LoadScene("InGame");
                //TODO switch to InGame State
                break;
            case State.NoChange:
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
        if(Lobby!=null)
            UpdateRoom();
        GameManager.Instance.GameTcpClient.Handle();
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

    public void ClearList()
    {
        for (var i = 0; i < playerListTransform.childCount; i++) Destroy(playerListTransform.GetChild(i).gameObject);
    }
    
    private void OnDisable()
    {
        ClearList();
    }
}
