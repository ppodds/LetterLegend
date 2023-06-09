using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class ExitGame : MonoBehaviour
{
    public async void Exit()
    {
        await GameManager.Instance.GameTcpClient.Exit();
        GameManager.Instance.QuitGame();
    }
}
