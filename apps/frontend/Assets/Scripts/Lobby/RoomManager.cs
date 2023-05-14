using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
public class RoomManager : MonoBehaviour
{
    public GameObject startPanel;
    public GameObject lobbyPanel;
    public GameObject roomPanel;
    public Button backButton;
    // Start is called before the first frame update
    void Awake()
    {
        backButton.onClick.AddListener(SwitchToLobby);
    }
    void SwitchToLobby()
    {
        startPanel.SetActive(false);
        lobbyPanel.SetActive(true);
        roomPanel.SetActive(false);
    }
    // Update is called once per frame
    void Update()
    {
        
    }
}
