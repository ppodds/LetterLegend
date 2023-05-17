using System.Collections;
using System.Collections.Generic;
using Protos.Lobby;
using Protos.Player;
using TMPro;
using UnityEngine;

public class PlayerItem : MonoBehaviour
{
    private TMP_Text _name;
    private TMP_Text _id;

    public void SetText(Lobby lobby, Player p)
    {
        //TODO Make lead object
        // lead.gameObject.SetActive(lobby.Lead.Id == p.Id);
        _name.SetText(p.Name);
        _id.SetText(p.Id.ToString());
    }
}
