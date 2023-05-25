using UnityEngine;

public class FinishTurn : MonoBehaviour
{
    public Timer timer;
    
    public async void Finish()
    {
        await GameManager.Instance.GameTcpClient.FinishTurn();
        timer.ResetCurrentTime();
    }
}