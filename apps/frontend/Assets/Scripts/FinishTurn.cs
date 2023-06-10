using UnityEngine;

public class FinishTurn : MonoBehaviour
{
    public Timer timer;
    private HandField _handField;
    public PlayerShowText playerShowText;

    private void Awake()
    {
        _handField = HandField.GetInstance();
    }

    public async void Finish()
    {
        var res = await GameManager.Instance.GameTcpClient.FinishTurn();
        if (res != null)
        {
            timer.ResetCurrentTime();
            _handField.SetHandField(res.Item1);
            playerShowText.SetPlayerName(res.Item2, res.Item3);
            GameManager.Instance.SetPlayers(res.Item2, res.Item3);
        }
    }
}