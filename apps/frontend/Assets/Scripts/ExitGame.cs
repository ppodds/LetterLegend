using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

public class ExitGame : MonoBehaviour
{
    public async void Exit()
    {
        await GameManager.Instance.GameTcpClient.Exit();
        SceneManager.LoadScene(0);
    }
}
