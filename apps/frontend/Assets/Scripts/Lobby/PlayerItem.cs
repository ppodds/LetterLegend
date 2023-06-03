using System.Collections;
using System.Collections.Generic;
using Protos.Lobby;
using Protos.Player;
using TMPro;
using UnityEngine;

public class PlayerItem : MonoBehaviour
{
    public TMP_Text playerName;
    public TMP_Text id;

    public void SetText(Lobby lobby, Player p)
    {
        //TODO Make lead object
        playerName.SetText(p.Name);
        id.SetText(p.Id.ToString());
    }
}