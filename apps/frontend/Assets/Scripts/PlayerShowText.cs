using System.Collections;
using Protos.Player;
using TMPro;
using UnityEngine;
using Random = UnityEngine.Random;

public class PlayerShowText : MonoBehaviour
{
    public TextMeshProUGUI textMeshProUGUI;
    private Camera _camera;

    private void Awake()
    {
        _camera = Camera.main;
        SetPlayerName(GameManager.Instance.GetCurrentPlayer(), GameManager.Instance.GetNextPlayer());
    }

    public void SetPlayerName(Player player1, Player player2)
    {
        if (Equals(player1, GameManager.Instance.GetMainPlayer()))
        {
            StartCoroutine(Shake());
        }
        textMeshProUGUI.text = "current player: " + player1.Name + "\nnext player: " + player2.Name;
    }

    private IEnumerator Shake()
    {
        var originalPosition = _camera.transform.position;
        for(var i = 0;i < 15; i ++)
        {
            var x = 0f;
            var y = 0f;
            if (i % 2 == 0)
            {
                x = Random.Range(0f, 1f);
                y = Random.Range(0f, 1f);
            }
            else
            {
                x = Random.Range(-1f, 0f);
                y = Random.Range(-1f, 0f);
            }
            _camera.transform.position = new Vector3(originalPosition.x + x, originalPosition.y + y, originalPosition.z);
            yield return new WaitForSeconds(0.05f);
        }
        _camera.transform.position = originalPosition;
    }
}