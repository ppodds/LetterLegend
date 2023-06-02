using TMPro;
using UnityEngine;

public class Timer : MonoBehaviour
{
    private float _currentTime;
    private float _currentTimeBase;
    public TextMeshProUGUI textMeshProUGUI;

    private void Awake()
    {
        _currentTime = _currentTimeBase = Time.time;
    }

    private void Update()
    {
        _currentTime = Time.time;
        if (_currentTime - _currentTimeBase >= 30)
        {
            _currentTimeBase = _currentTime;
        }

        textMeshProUGUI.SetText(((int)(30 - (_currentTime - _currentTimeBase) + 0.5)).ToString());
    }

    public void ResetCurrentTime()
    {
        _currentTimeBase = _currentTime;
    }
}