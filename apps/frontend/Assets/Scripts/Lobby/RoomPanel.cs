using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
public class RoomPanel : MonoBehaviour
{
    public GameObject startPanel;
    public GameObject lobbyPanel;
    public GameObject roomPanel;
    public GameObject playerItem;
    public Transform playerListTransform;

    public void SwitchToLobby()
    {
        startPanel.SetActive(false);
        lobbyPanel.SetActive(true);
        roomPanel.SetActive(false);
    }
    
    private void Awake()
    {
        for (int i = 0; i < 5; i++)
        {
            var t = Instantiate(playerItem, playerListTransform).GetComponent<PlayerItem>();
            t.GetComponent<Button>().onClick.AddListener(SwitchToGame);
        }
    }
    
    public void SwitchToGame()
    {
        
    }
}
