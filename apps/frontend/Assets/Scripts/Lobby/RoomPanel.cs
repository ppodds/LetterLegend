using System.Collections;
using System.Collections.Generic;
using Protos.Lobby;
using UnityEngine;
using UnityEngine.UI;
public class RoomPanel : MonoBehaviour
{
    public GameObject startPanel;
    public GameObject lobbyPanel;
    public GameObject roomPanel;
    public GameObject playerItem;
    public Transform playerListTransform;
    public Lobby Lobby { get; set; }

    public async void BackToLobby()
    {
        await GameManager.Instance.GameTcpClient.QuitLobby();
        // GameManager.Instance.lobbyPanel.gameObject.SetActive(true);
        lobbyPanel.SetActive(true);
        gameObject.SetActive(false);
    }
    
    public void UpdateRoom()
    {
        Debug.Log(Lobby.Players.Count);
        foreach (var player in Lobby.Players)
        {
            Debug.Log(player.Id + player.Name);
            var t = Instantiate(playerItem, playerListTransform).GetComponent<PlayerItem>();
            t.SetText(Lobby, player);
        }
    }

    public async void StartGame()
    {
        var ready = await GameManager.Instance.GameTcpClient.Ready();
        if (!ready)
        {
            Debug.Log("start game fail");
        }
        GameManager.Instance.StartGame();
    }
    
    private void OnDisable()
    {
        for (var i = 0; i < playerListTransform.childCount; i++) Destroy(playerListTransform.GetChild(i).gameObject);
    }
    
    public void SwitchToGame()
    {
        
    }
    
}
