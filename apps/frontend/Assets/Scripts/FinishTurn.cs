using UnityEngine;

public class FinishTurn : MonoBehaviour
{
    private Timer _timer;

    public void Awake()
    {
        _timer = Timer.GetInstance();
    }

    public async void Finish()
    {
        await GameManager.Instance.GameTcpClient.FinishTurn();
        _timer.ResetCurrentTime();
    }
}