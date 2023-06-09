using System.Collections;
using Protos.Player;
using TMPro;
using UnityEngine;
using Random = UnityEngine.Random;

public class PlayerShowText : MonoBehaviour
{
    public TextMeshProUGUI textMeshProUGUI;
    private Camera _camera;
    private float _startTime;

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
        _startTime = Time.time;
        var originalPosition = _camera.transform.position;
        while (Time.time - _startTime < 0.4f)
        {
            var x = Random.Range(-0.25f, 0.25f);
            var y = Random.Range(-0.25f, 0.25f);
            _camera.transform.position = new Vector3(originalPosition.x + x, originalPosition.y + y, originalPosition.z);
            yield return new WaitForSeconds(0.075f);
        }
        _camera.transform.position = originalPosition;
    }
}