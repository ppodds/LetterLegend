using UnityEngine;

public class FinishTurn : MonoBehaviour
{
    public Timer timer;
    private HandField _handField;

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
            _handField.SetHandField(res);
        }
    }
}