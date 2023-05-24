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
    public Button readyButton;
    public Lobby Lobby { get; set; }

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
            Debug.Log(player.Id + player.Name);
            var t = Instantiate(playerItem, playerListTransform).GetComponent<PlayerItem>();
            t.SetText(Lobby, player);
        }
    }
    
    public async void SetReady()
    {
        var res = await GameManager.Instance.GameTcpClient.SetReady();
        if (res)
        {
            readyButton.image.color = Color.gray;
        }
        else
        {
            Debug.Log("Set ready failed");
        }
    }
    
    public void StartGame()
    {
        GameManager.Instance.StartGame();
    }
    
    private void OnDisable()
    {
        for (var i = 0; i < playerListTransform.childCount; i++) Destroy(playerListTransform.GetChild(i).gameObject);
    }
}
