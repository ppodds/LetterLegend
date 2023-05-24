using TMPro;
using UnityEngine;

public class Timer : MonoBehaviour
{
    private static Timer _timer;
    private float _heartBeatTimeBase;
    private float _heartBeatTime;
    private float _currentTime;
    public TextMeshProUGUI textMeshProUGUI;
    private void Awake()
    {
        if (_timer != null && _timer != this)
        {
            Destroy(gameObject);
            return;
        }

        _timer = this;
        _heartBeatTimeBase = _heartBeatTime = 0;
        _currentTime = 0;
    }

    private void Update()
    {
        _currentTime -= Time.deltaTime;
        if (_currentTime <= 0)
        {
            _currentTime = 30;
        }
        textMeshProUGUI.SetText(((int) (_currentTime + 0.5)).ToString());
        
        _heartBeatTime += Time.deltaTime;
        if (_heartBeatTime - _heartBeatTimeBase >= 20)
        {
            HeartBeat();
            _heartBeatTimeBase = _heartBeatTime = 0;
        }
    }

    private async void HeartBeat()
    {
        await GameManager.Instance.GameTcpClient.HeartBeat();
    }

    public void ResetCurrentTime()
    {
        _currentTime = 30;
    }

    public static Timer GetInstance()
    {
        return _timer;
    }
}
