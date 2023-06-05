using System.Collections;
using System.Collections.Generic;
using Protos.Game;
using Protos.Player;
using TMPro;
using UnityEngine;

public class PlayerShowText : MonoBehaviour
{
    public TextMeshProUGUI textMeshProUGUI;
    private void Awake()
    {
        SetPlayerName(GameManager.Instance.GetPlayers());
    }

    public void SetPlayerName(List<Player> players)
    {
        textMeshProUGUI.text = "current player: " + players[0].Name + "\nnext player: " + players[1].Name;
    }

    public void SetPlayerName(Player player1, Player player2)
    {
        textMeshProUGUI.text = "current player: " + player1.Name + "\nnext player: " + player2.Name;
    }
}
