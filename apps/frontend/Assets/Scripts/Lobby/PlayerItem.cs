using System.Collections;
using System.Collections.Generic;
using Protos.Lobby;
using Protos.Player;
using TMPro;
using UnityEngine;

public class PlayerItem : MonoBehaviour
{
    public TMP_Text name;
    public TMP_Text id;

    public void SetText(Lobby lobby, Player p)
    {
        //TODO Make lead object
        // lead.gameObject.SetActive(lobby.Lead.Id == p.Id);
        name.SetText(p.Name);
        id.SetText(p.Id.ToString());
    }
}
